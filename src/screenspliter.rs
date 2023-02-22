/*

Copyright (c) 2022, Corentin JUVIGNY

Permission to use, copy, modify, and/or distribute this software
for any purpose with or without fee is hereby granted, provided
that the above copyright notice and this permission notice appear
in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL
WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR
CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

*/

use std::{process::Command, str::FromStr, fmt};

#[derive(Clone, Copy, clap::ValueEnum, Debug)]
pub enum Position {
   Left,
   Same,
   Right
}

impl Default for Position {
   fn default() -> Self
   {
      Self::Right
   }
}

impl fmt::Display for Position {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
   {
      write!(f,"{:#?}",self)
   }
}

impl Position {
   fn as_xrandr_arg(&self) -> &str
   {
      match self {
         Self::Left  => "--left-of",
         Self::Same  => "--same-as",
         Self::Right => "--right-of"
      }
   }
}

#[derive(Debug)]
struct Monitor {
   name: String,
   resolution: (u32,u32),
}

impl Monitor {
   fn new(name: &str, resolution: &(u32,u32)) -> Self
   {
      Monitor { name: String::from(name)
              , resolution: resolution.clone() }
   }

   fn name(&self) -> &str
   {
      self.name.as_ref()
   }

   fn to_string(&self) -> String
   {
      format!("{}x{}",self.resolution.0,self.resolution.1)
   }
}

#[derive(Debug, Clone)]
struct NoMonitorFound;

impl fmt::Display for NoMonitorFound {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "No monitor has been found")
   }
}

impl std::error::Error for NoMonitorFound { }

fn get_available_monitors() -> Result<Vec<Monitor>,Box<dyn std::error::Error>>
{
   let output = Command::new("sh").args(["-c","xrandr | grep -A1 \" connected \" | awk '{ if ($1 != \"--\") print $1; }'"]).output()?;
   let stdout = String::from_utf8(output.stdout)?;
   let lines = stdout.lines().map(str::trim).collect::<Vec<&str>>();
   Ok((0..lines.len()).step_by(2).map(|i| {
         if let [resx,resy] = lines[i+1].split('x')
                                        .map(u32::from_str)
                                        .filter(Result::is_ok)
                                        .map(Result::unwrap)
                                        .collect::<Vec<_>>()[..2] {
            Some(Monitor::new(lines[i],&(resx,resy)))
         } else {
            None
         }
      })
      .filter(Option::is_some)
      .map(Option::unwrap)
      .collect::<Vec<Monitor>>())
}

pub fn list_monitors() -> Result<(),Box<dyn std::error::Error>>
{
   let monitors = get_available_monitors()?;
   if monitors.len() > 0 {
      println!("Available monitors:");
      for monitor in monitors {
         println!("{}: {}x{}",monitor.name,monitor.resolution.0,monitor.resolution.1);
      }
      Ok(())
   } else {
      Err(NoMonitorFound.into())
   }
}

pub fn split_screen(path: &str, position: &Position) -> Result<(),Box<dyn std::error::Error>>
{
   match &get_available_monitors()?[..] {
      [primary_monitor,auxiliary_monitor,..] => {
         Command::new("xrandr").args(["--output",primary_monitor.name()])
                               .args(["--mode",primary_monitor.to_string().as_str()])
                               .args(["--output",auxiliary_monitor.name()])
                               .args(["--mode",auxiliary_monitor.to_string().as_str()])
                               .args([position.as_xrandr_arg(),primary_monitor.name()])
                               .output()?;
         Command::new("feh").arg("--bg-scale").arg(path).output().map(|_| ()).map_err(|err| err.into())
      },
      [_] => Ok(()),
      [] => Err(NoMonitorFound.into())
   }
}
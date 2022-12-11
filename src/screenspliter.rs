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

   fn resolution_str(&self) -> String
   {
      format!("{}x{}",self.resolution.0,self.resolution.1)
   }
}

fn get_available_monitors() -> Result<Vec<Monitor>,Box<dyn std::error::Error>>
{
   let output = Command::new("sh").args(["-c","xrandr | grep -A1 \" connected \" | awk '{ if ($1 != \"--\") print$1; }'"]).output()?;
   let stdout = String::from_utf8(output.stdout)?;
   let lines = stdout.lines().map(|l| l.trim()).collect::<Vec<&str>>();
   let mut monitors = Vec::with_capacity(stdout.len());
   let mut i = 0;
   while i < lines.len() {
      let name = lines[i];
      let resolution = lines[i+1].split("x").map(|s| u32::from_str(s).unwrap_or_default()).collect::<Vec<u32>>();
      monitors.push(Monitor::new(name,&(resolution[0],resolution[1])));
      i += 2;
   }
   Ok(monitors)
}

pub fn split_screen(path: &str, position: &Position) -> Result<(),Box<dyn std::error::Error>>
{
   let available_monitors = get_available_monitors()?;
   if available_monitors.len() > 1 {
      Command::new("xrandr").args(["--output",available_monitors[0].name()])
                            .args(["--mode",available_monitors[0].resolution_str().as_str()])
                            .args(["--output",available_monitors[1].name()])
                            .args(["--mode",available_monitors[1].resolution_str().as_str()])
                            .args([position.as_xrandr_arg(),available_monitors[0].name()])
                            .output()?;
      Command::new("feh").arg("--bg-scale").arg(path).output()?;
   }
   Ok(())
}
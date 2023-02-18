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

use clap::{Parser, arg, command};
use dirs::home_dir;
use crate::screenspliter::Position;

fn wallpaper_default_value() -> String
{
   home_dir().map(|d| match d.to_str() {
               Some(s) => String::from(s),
               None => String::default()
             })
             .map(|s| s + "/.local/screensplit/wallpaper/renoir_sunset_at_sea.png")
             .unwrap_or_default()
}

/// Screen spliter - get the first two connected screens and set the second at the given position
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Path to the wallpaper (Warning : it has to be an absolute path)
   #[arg(short, long, default_value_t = wallpaper_default_value())]
   pub wallpaper_path: String,

   /// Relative position of the second screen
   #[arg(short, long, value_enum, default_value_t = Position::default())]
   pub position: Position
}
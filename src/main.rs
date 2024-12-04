mod advent1;
mod advent2;
use clap::{Arg, Command};

use crate::advent1::advent1;
use crate::advent2::advent2;
fn main() {
    let matches = Command::new("aoc")
        .version("1.0")
        .author("Geoffrey White")
        .about("cli to call my advent of code functions")
        .arg(Arg::new("day").short('d').long("day"))
        .arg(Arg::new("part").short('p').long("part"))
        .get_matches();

    // let id: &str;
    let mut num: i32 = 0;
    if let Some(c) = matches.get_one::<String>("day") {
        if let Ok(checked_num) = c.parse::<i32>() {
            num = checked_num;
        }
    }
    match num {
        1 => {
            advent1();
        }
        2 => {
            advent2();
        }
        _ => {}
    }
}

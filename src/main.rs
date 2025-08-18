mod advent1;
mod advent2;
mod advent3;
mod advent4;
mod advent5;
mod advent6;
mod advent7;
use clap::{Arg, Command};

use crate::advent1::advent1;
use crate::advent2::advent2;
use crate::advent3::advent3;
use crate::advent4::advent4;
use crate::advent5::advent5;
use crate::advent6::advent6;
use crate::advent7::advent7;
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
        3 => {
            advent3();
        }
        4 => {
            advent4();
        }
        5 => {
            advent5();
        }
        6 => {
            advent6();
        }
        7 => {
            advent7();
        }
        _ => {}
    }
}

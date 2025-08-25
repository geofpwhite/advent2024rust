mod advent1;
mod advent10;
mod advent11;
mod advent12;
mod advent13;
mod advent2;
mod advent3;
mod advent4;
mod advent5;
mod advent6;
mod advent7;
mod advent8;
mod advent9;
use clap::{Arg, Command};

use crate::advent1::advent1;
use crate::advent10::advent10;
use crate::advent11::advent11;
use crate::advent12::advent12;
use crate::advent13::advent13;
use crate::advent2::advent2;
use crate::advent3::advent3;
use crate::advent4::advent4;
use crate::advent5::advent5;
use crate::advent6::advent6;
use crate::advent7::advent7;
use crate::advent8::advent8;
use crate::advent9::advent9;
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
        8 => {
            advent8();
        }
        9 => {
            advent9();
        }
        10 => {
            advent10();
        }
        11 => {
            advent11();
        }
        12 => {
            advent12();
        }
        13 => {
            advent13();
        }
        _ => {}
    }
}

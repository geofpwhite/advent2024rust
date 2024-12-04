use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub(crate) fn advent2() {
    let Ok(mut file) = File::open("inputs/advent2.txt") else { return };
    let mut contents = String::new();
    let _none = file.read_to_string(&mut contents).unwrap_or(0);
    let parts: Vec<&str> = contents.split('\n').collect();
    println!("{parts:?}");

}

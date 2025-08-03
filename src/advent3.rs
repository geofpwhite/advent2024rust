use regex::Regex;
use std::fs::File;
use std::io::Read;

pub(crate) fn advent3() {
    let mut file: File = File::open("inputs/advent3.txt").unwrap();
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    file.read_to_string(&mut contents).unwrap();
    let p1 = part1(&contents,re.clone());
    println!("{p1}");
    let trim_re = Regex::new(r"[^do|^don't|^mul([0-9]{1,3},[0-9]{1,3})]").unwrap();
    let re2 = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let re3 = Regex::new(r"don't\(\).*").unwrap();
    contents = trim_re.replace_all(&contents, "").to_string();
    contents = re2.replace_all(&contents, "").to_string();
    // contents = re2.replace_all(&contents, "").to_string();
    contents = re3.replace_all(&contents, "").to_string();
    // println!("{contents}");
    let p2 = part1(contents.as_str(), re);
    println!("{p2}");
    // println!("{sum}");
}

fn part1(contents: &str, re: Regex) -> i32 {
    let nums = re
        .captures_iter(&contents)
        .map(|cap| {
            cap.iter()
                .skip(1)
                .map(|s| s.unwrap().as_str().parse::<i32>())
                .map(Result::unwrap)
                .into_iter()
                .product::<i32>()
        })
        .sum::<i32>();

    nums
}

use regex::Regex;
use std::fs::File;
use std::io::Read;

pub(crate) fn advent3() {
    let Ok(mut file) = File::open("inputs/advent3.txt") else {
        return;
    };
    let re = Regex::new(r"mul\(([0-9]{1,3})+,([0-9]{1,3})+\)").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut results = vec![];
    for (line, [_, _]) in re.captures_iter(&contents).map(|c| c.extract()) {
        results.push(line);
    }
    let x: &[_] = &['m', 'u', 'l', '(', ')'];
    let nums = results
        .iter()
        .map(|line| {
            let binding = line.to_string();
            let mut num_strings = binding.trim_matches(x).split(",");
            num_strings
                .next()
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()
                * num_strings
                    .next()
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap()
        })
        .sum::<i32>();
    // let s: i32 = nums.sum();
    println!("{nums}");
}

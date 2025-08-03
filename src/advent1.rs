use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Result;

pub(crate) fn advent1() {
    let x: String = read_file_to_string("./inputs/advent1.txt").unwrap();
    let parts: Vec<&str> = x.split('\n').collect();
    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];
    for part in parts {
        let nums: Vec<&str> = part.split("   ").collect();
        // let Ok(num) = nums[0].parse::<i32>() else {continue};
        let num = nums[0].parse::<i32>().unwrap();
        l1.push(num);
        let Ok(num) = nums[1].parse::<i32>() else {continue};
        l2.push(num);
    }
    l1.sort();
    l2.sort();
    let sum = l1
        .iter()
        .zip(l2.iter())
        .fold(0, |acc, x: (&i32, &i32)| match x.0 > x.1 {
            true => return acc + x.0 - x.1,
            false => return acc + x.1 - x.0,
        });
    println!("{:?}", sum);
    //part1 complete
    let mut counts: HashMap<i32, i32> = HashMap::new();
    l2.iter().for_each(|i: &i32| {
        *counts.entry(*i).or_default() += 1;
    });
    let sum2: i32 = l1
        .iter()
        .map(|i: &i32| counts.get(&i).unwrap_or(&0) * *i)
        .sum();
    println!("{sum2}")
    // sum = 0;
}

fn read_file_to_string(filename: &str) -> Result<String> {
    let mut file: std::fs::File = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

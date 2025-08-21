use std::{collections::HashMap, fs::File, io::Read};

pub(crate) fn advent11() {
    let mut buf = String::from("");
    File::open("inputs/advent11.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    let mut nums: Vec<i64> = buf
        .trim()
        .split(" ")
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();
    let hold = nums.clone();
    for _ in 0..25 {
        nums = multiply(nums);
    }
    println!("{}", nums.len());
    let mut nums_map: HashMap<i64, i64> = HashMap::new();
    hold.iter().for_each(|num| {
        let m = nums_map.get(num).unwrap_or(&0) + 1;
        nums_map.insert(*num, m);
    });
    for _ in 0..75 {
        nums_map = multiply2(nums_map);
    }
    let mut sum = 0;
    nums_map.iter().for_each(|(_, freq)| sum += freq);
    println!("{sum}");
}

fn multiply(nums: Vec<i64>) -> Vec<i64> {
    let mut return_nums: Vec<i64> = vec![];
    nums.iter().for_each(|num| match num.to_string().len() % 2 {
        0 => {
            let ns = num.to_string();
            let n1 = ns
                .chars()
                .take(ns.len() / 2)
                .fold(String::from(""), |acc: String, n: char| {
                    let mut acc_str = acc.clone().to_string();
                    acc_str.push(n.clone());
                    acc_str
                })
                .parse::<i64>()
                .unwrap();
            let n2 = ns
                .chars()
                .skip(ns.len() / 2)
                .fold(String::from(""), |acc: String, n: char| {
                    let mut acc_str = acc.clone().to_string();
                    acc_str.push(n.clone());
                    acc_str
                })
                .parse::<i64>()
                .unwrap();
            return_nums.push(n1);
            return_nums.push(n2);
        }
        _ => {
            if *num == 0 {
                return_nums.push(1);
            } else {
                return_nums.push(num * 2024)
            }
        }
    });
    return_nums
}

fn multiply2(nums: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut return_nums = HashMap::new();
    nums.iter()
        .for_each(|(num, frequency)| match num.to_string().len() % 2 {
            0 => {
                let ns = num.to_string();
                let n1 = ns
                    .chars()
                    .take(ns.len() / 2)
                    .fold(String::from(""), |acc: String, n: char| {
                        let mut acc_str = acc.clone().to_string();
                        acc_str.push(n.clone());
                        acc_str
                    })
                    .parse::<i64>()
                    .unwrap();
                let n2 = ns
                    .chars()
                    .skip(ns.len() / 2)
                    .fold(String::from(""), |acc: String, n: char| {
                        let mut acc_str = acc.clone().to_string();
                        acc_str.push(n.clone());
                        acc_str
                    })
                    .parse::<i64>()
                    .unwrap();
                let m = return_nums.get(&n1).unwrap_or(&0) + frequency;
                return_nums.insert(n1, m);
                let m = return_nums.get(&n2).unwrap_or(&0) + frequency;
                return_nums.insert(n2, m);
            }
            _ => {
                if *num == 0 {
                    let m = return_nums.get(&1).unwrap_or(&0) + frequency;
                    return_nums.insert(1, m);
                } else {
                    let m = return_nums.get(&(num * 2024)).unwrap_or(&0) + frequency;
                    return_nums.insert(num * 2024, m);
                }
            }
        });
    return_nums
}

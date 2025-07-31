use std::fs::File;
use std::io::{Read, Split};
use std::iter::{Enumerate, Map};

pub(crate) fn advent2() {
    let Ok(mut file) = File::open("inputs/advent2.txt") else {
        return;
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let parts: Vec<&str> = contents.split('\n').filter(|s| *s != "").collect();
    let count_part_1 = parts
        .iter()
        .filter(|s| {
            let mut string_nums = s.split(" ").enumerate();
            let first = string_nums.next().unwrap();
            let mut prev = first.1.to_string().parse::<i32>().unwrap();
            let mut dir = 0;

            for sub in string_nums {
                if let Ok(num) = sub.1.to_string().parse::<i32>() {
                    if sub.0 == 1 {
                        if prev < num {
                            dir = 1;
                        } else {
                            dir = -1;
                        }
                    }
                    if (num - prev) * dir <= 0 || (num - prev) * dir > 3 {
                        return false;
                    }
                    prev = num
                }
            }
            true
        })
        .count();
    println!("{count_part_1}");

    let count_part_2 = parts
        .iter()
        .filter(|s| {
            let nums = s
                .split(" ")
                .map(|s| (s.to_string().parse::<i32>().unwrap()));
            if valid(nums.clone().collect()) {
                return true;
            }
            for i in nums.clone().enumerate() {
                let mut cl = nums.clone().skip(i.0 as usize).collect::<Vec<i32>>();
                if i.0 != 0 {
                    let mut cl2 = nums.clone().take(i.0 - 1 as usize).collect::<Vec<i32>>();
                    cl2.append(&mut cl);
                    cl = cl2;
                }
                println!("{:?}", cl);
                if valid(cl) {
                    return true;
                }
            }

            false
        })
        .count();
    println!("{count_part_2}");
}

fn valid(nums: Vec<i32>) -> bool {
    let nums = nums.iter().enumerate();
    let mut prev = nums.clone().next().unwrap().1;
    let mut dir = 0;
    for num in nums {
        if num.0 == 1 {
            if prev < num.1 {
                dir = 1;
            } else {
                dir = -1;
            }
        }
        if (*num.1 - *prev) * dir <= 0 || (*num.1 - *prev) * dir > 3 {
            return false;
        }
        prev = num.1
    }
    true
}

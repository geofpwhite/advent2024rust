use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
};

pub(crate) fn advent19() {
    let mut contents = String::new();
    File::open("inputs/advent19.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    let mut lines = contents.lines();
    let towels = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
    let mut count = 0;
    let mut count2 = 0;
    let mut valids: HashMap<&&str, usize> = HashMap::new();
    lines.clone().filter(|l| l != &"").for_each(|line| {
        if valid(line.trim(), &towels) {
            count += 1;
            // count2 += valid2(line.trim(), &towels, &valids);
            // println!("{count}")
        }
    });
    lines.clone().filter(|l| l != &"").for_each(|line| {
        count2 += valid2(line.trim(), &towels, &mut HashMap::new(), 0);
        println!("{count2}")
    });

    println!("{count} {count2}")
}

fn valid(pattern: &str, towels: &Vec<&str>) -> bool {
    let mut queue = vec![0];
    while queue.len() > 0 {
        let pattern_index = queue.pop().unwrap();
        if pattern_index == pattern.len() {
            return true;
        }

        towels.iter().for_each(|towel| {
            if towel.len() + pattern_index <= pattern.len()
                && pattern
                    .chars()
                    .skip(pattern_index)
                    .take(towel.len())
                    .collect::<String>()
                    == towel.to_string()
            {
                queue.push(pattern_index + towel.len());
            }
        });
    }

    false
}

fn valid2(
    pattern: &str,
    towels: &Vec<&str>,
    dp: &mut HashMap<String, i64>,
    mut value: usize,
) -> usize {
    if let Some(num) = dp.get(&pattern.to_string()) {
        return {
            if *num == -1 {
                return 0;
            }
            *num as usize
        };
    }
    let towels = towels.clone();
    if let Some(num) = dp.get(&pattern.to_string()) {
        if *num == -1 {
            return 0;
        }
        return *num as usize;
    }
    for t in towels.clone() {
        if t == pattern {
            let x = dp.get(&t.to_string()).unwrap_or(&0) + 1;
            dp.insert(t.to_string(), x);
            value += 1;
        }
        if t.len() < pattern.len() && t == pattern.chars().take(t.len()).collect::<String>() {
            let v = valid2(
                &pattern.chars().skip(t.len()).collect::<String>(),
                &towels,
                dp,
                0,
            );
            if v > 0 {
                value += v;
                dp.insert(pattern.chars().skip(t.len()).collect::<String>(), v as i64);
            } else {
                dp.insert(pattern.chars().skip(t.len()).collect::<String>(), -1);
            }
        }
    }
    if value == 0 {
        dp.insert(pattern.to_string(), -1);
    }
    dp.insert(pattern.to_string(), value as i64);
    value
}

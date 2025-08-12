use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

pub(crate) fn advent5() {
    let mut contents = String::new();
    File::open("inputs/advent5.txt")
        // File::open("inputs/test5.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents = contents.replace("\r", "\n");
    let rlines = contents.split("\n").filter(|s| s.to_string() != "");
    let mut lines = rlines.clone();
    let mut rules: Hmap<i64> = HashMap::new();
    let mut line = lines.next();
    while line.unwrap().contains("|") {
        let rule: Vec<i64> = line
            .unwrap()
            .split("|")
            .map(|s| str::parse::<i64>(s).unwrap())
            .collect();

        if let Some(other_suffixes) = rules.get(&rule[0]) {
            let rule = rule;
            let mut other_suffixes = other_suffixes.clone();
            other_suffixes.insert(rule[1]);
            rules.insert(rule[0], other_suffixes);
        } else {
            let mut hset = HashSet::new();
            hset.insert(rule[1]);
            rules.insert(rule[0], hset);
        }
        line = lines.next();
    }
    let mut sum = 0;
    let mut sum2 = 0;
    'outer: for s in rlines.filter(|s| s.contains(",")) {
        let nums = s
            .split(",")
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .collect::<Vec<i64>>();
        for (i, j) in nums.iter().enumerate() {
            for num in nums.iter().skip(i) {
                if rules.contains_key(num) {
                    if rules.get(num).unwrap().contains(j) {
                        let nums = rules.sort(nums);
                        sum2 += nums[nums.len() / 2];
                        continue 'outer;
                    }
                }
            }
        }
        sum += nums[nums.len() / 2]
    }
    println!("part1: {:?}", sum);
    println!("part2: {:?}", sum2);
}

type Hmap<K> = HashMap<K, HashSet<K>>;
trait Hmappable<K> {
    fn sort(&self, nums: Vec<i64>) -> Vec<i64>;
}

impl Hmappable<i64> for Hmap<i64> {
    fn sort(&self, mut nums: Vec<i64>) -> Vec<i64> {
        nums.sort_by(|i, j| {
            if let Some(children) = self.get(i) {
                if children.contains(j) {
                    return Ordering::Less;
                }
            }
            if let Some(children) = self.get(j) {
                if children.contains(i) {
                    return Ordering::Greater;
                }
            }

            Ordering::Equal
        });
        nums
    }
}

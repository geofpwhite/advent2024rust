use std::{fs::File, io::Read};

pub(crate) fn advent7() {
    let mut contents = String::new();
    File::open("inputs/advent7.txt")
        // File::open("inputs/test6.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents = contents.replace("\r", "\n");
    let rlines = contents.split("\n").filter(|s| s.to_string() != "");
    let to_check = rlines
        .map(|s| {
            let split = s
                .replace(":", "")
                .split(" ")
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect::<Vec<i64>>();
            Equation::new(split[0], split[1..split.len()].to_vec())
        })
        .collect::<Vec<Equation>>();
    let mut sum = 0;
    let mut sum2 = 0;
    to_check.iter().for_each(|eq| {
        if eq.eval() {
            sum += eq.value;
        }
        if eq.eval2() {
            sum2 += eq.value;
        }
    });
    println!("{sum} {sum2}")
}

#[derive(Debug, Clone)]
struct Equation {
    value: i64,
    equals: Vec<i64>,
}

impl Equation {
    fn new(value: i64, equals: Vec<i64>) -> Self {
        Equation {
            value: value,
            equals: equals,
        }
    }
    fn eval(&self) -> bool where {
        let mut visited = vec![];
        visited.push(self.equals[0]);
        self.equals.iter().skip(1).for_each(|num| {
            let mut v1: Vec<i64> = visited.iter().map(|num2| num + num2).collect();
            let mut v2 = visited.iter().map(|num2| num * num2).collect();
            v1.append(&mut v2);
            visited = v1;
        });
        visited.iter().any(|num| *num == self.value)
    }
    fn eval2(&self) -> bool where {
        let mut visited = vec![];
        visited.push(self.equals[0]);
        self.equals.iter().skip(1).for_each(|num| {
            let mut v1: Vec<i64> = visited.iter().map(|num2| num + num2).collect();
            let mut v2 = visited.iter().map(|num2| num * num2).collect();
            let mut v3 = visited
                .iter()
                .map(|num2| (num2.to_string() + num.to_string().as_str()).parse::<i64>())
                .map(Result::unwrap)
                .collect();
            v1.append(&mut v2);
            v1.append(&mut v3);
            visited = v1;
        });
        visited.iter().any(|num| *num == self.value)
    }
}

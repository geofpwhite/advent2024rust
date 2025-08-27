use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn advent14() {
    let mut q1 = vec![];
    let mut q2 = vec![];
    let mut q3 = vec![];
    let mut q4 = vec![];
    let mut max_x = 101;
    let mut max_y = 103;
    let binding = parse().unwrap();
    let mut robots = binding.iter().map(|robot| {
        let mut new_px = (robot.p[0] + (100 * robot.v[0])) % max_x;
        let mut new_py = (robot.p[1] + (100 * robot.v[1])) % max_y;
        if new_px < 0 {
            new_px = max_x + new_px
        }
        if new_py < 0 {
            new_py = max_y + new_py
        }
        let mut r = robot.clone();
        r.p = [new_px, new_py];
        r
    });
    robots.for_each(|robot| {
        let mx = max_x / 2;
        let my = max_y / 2;
        if robot.p[0] < mx && robot.p[1] < my {
            q1.push(robot);
        }
        if robot.p[0] > mx && robot.p[1] < my {
            q2.push(robot);
        }
        if robot.p[0] < mx && robot.p[1] > my {
            q3.push(robot);
        }
        if robot.p[0] > mx && robot.p[1] > my {
            q4.push(robot);
        }
    });
    println!("{}", q1.len() * q2.len() * q3.len() * q4.len());
    let mut robots = parse().unwrap();
    let overlapping_points = |mut robots: &Vec<Robot>| {
        let mut hset = HashSet::new();
        for r in robots {
            if hset.contains(&r.p) {
                return true;
            }
            hset.insert(r.p);
        }
        false
    };
    (1..10000).for_each(|i| {
        robots = robots
            .iter()
            .map(|r| {
                let mut r = r.clone();
                r.p[0] += r.v[0];
                r.p[0] %= max_x;
                if r.p[0] < 0 {
                    r.p[0] = max_x + r.p[0];
                }
                r.p[1] += r.v[1];
                r.p[1] %= max_y;
                if r.p[1] < 1 {
                    r.p[1] = max_y + r.p[1];
                }
                r
            })
            .collect();
        if !overlapping_points(&robots) {
            println!("{i}");
        }
    });
}

fn parse() -> Result<(Vec<Robot>), Box<dyn Error>> {
    let file = File::open("inputs/advent14.txt")?;
    let reader = BufReader::new(file);

    let mut robots = vec![];
    for line in reader.lines() {
        let l = line?; // ugly
        let mut words = l.split_whitespace();
        let mut r = Robot::new();
        let p = words
            .next()
            .unwrap()
            .replace("p=", "")
            .split(",")
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .collect::<Vec<i64>>();
        r.p = [p[0], p[1]];
        let v = words
            .next()
            .unwrap()
            .replace("v=", "")
            .split(",")
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .collect::<Vec<i64>>();
        r.v = [v[0], v[1]];
        robots.push(r);
    }

    Ok(robots)
}

#[derive(Copy, Clone)]
struct Robot {
    p: Coords,
    v: Coords,
}
impl Robot {
    fn new() -> Self {
        Robot {
            p: [0, 0],
            v: [0, 0],
        }
    }
}

type Coords = [i64; 2];

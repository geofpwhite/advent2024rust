use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{self, Read},
};

pub(crate) fn advent18() {
    let mut contents = String::new();
    File::open("inputs/advent18.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    let mut hs = HashSet::new();
    contents
        .lines()
        .map(|s| s.split(","))
        .map(|mut s| {
            (
                str::parse::<usize>(s.next().unwrap()).unwrap(),
                str::parse::<usize>(s.next().unwrap()).unwrap(),
            )
        })
        .take(1024)
        .for_each(|(x, y)| {
            hs.insert((x, y));
        });
    let mut path = part1(&hs, true);
    let mut i = 1024;
    contents
        .lines()
        .map(|s| s.split(","))
        .map(|mut s| {
            (
                str::parse::<usize>(s.next().unwrap()).unwrap(),
                str::parse::<usize>(s.next().unwrap()).unwrap(),
            )
        })
        .skip(1024)
        .for_each(|(x, y)| {
            hs.insert((x, y));
            if path.contains(&(x, y)) {
                path = part1(&hs, false);
                if path.len() == 0 {
                    println!("{x} {y}");
                    return;
                }
            }
            i += 1;
        });
}

fn in_bounds(x: i32, y: i32) -> bool {
    x <= 70 && y <= 70 && x >= 0 && y >= 0
}

fn part1(hs: &HashSet<(usize, usize)>, print_part1: bool) -> HashSet<(usize, usize)> {
    let mut stack: Vec<(i32, i32, i32, HashSet<(usize, usize)>)> = vec![(0, 0, 0, HashSet::new())];
    let mut visited = HashMap::new();
    let mut path = HashSet::new();
    while stack.len() > 0 {
        let (x, y, steps, mut stack_path) = stack.pop().unwrap();
        visited.insert((x, y), steps);
        stack_path.insert((x as usize, y as usize));
        let sp = &stack_path;
        for (x2, y2) in [(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)] {
            if in_bounds(x2, y2)
                && !hs.contains(&(x2 as usize, y2 as usize))
                && ((!visited.contains_key(&(x2, y2)))
                    || (visited.get(&(x2, y2)).unwrap() > &(steps + 1)))
            {
                stack.push((x2, y2, steps + 1, sp.clone()));
                if (x2, y2) == (70, 70) {
                    path = sp.clone();
                }
            }
        }
    }
    if None == visited.get(&(70, 70)) {
        return HashSet::new();
    } else if print_part1 {
        println!("{}", visited.get(&(70, 70)).unwrap());
    }
    path
}

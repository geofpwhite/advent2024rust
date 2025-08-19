use std::{
    collections::{self, binary_heap::Iter, HashSet},
    fs::File,
    io::Read,
    iter,
};

pub(crate) fn advent10() {
    let mut contents = String::new();
    File::open("inputs/advent10.txt")
        // File::open("inputs/test9.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    let _b = contents.replace("\r", "");
    let lines = _b.trim().split("\n");
    let mut g = Grid::new();
    lines.for_each(|line| {
        g.grid
            .push(line.replace("\n", "").chars().collect::<Vec<char>>())
    });
    let mut sum = 0;
    for (i, v) in g.grid.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if *c == '0' {
                sum += g.score(i as i64, j as i64, true);
            }
        }
    }
    println!("{sum}");
    sum = 0;
    for (i, v) in g.grid.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if *c == '0' {
                sum += g.score(i as i64, j as i64, false);
            }
        }
    }
    println!("{sum}");
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
}
impl Grid {
    fn new() -> Self {
        return Self { grid: vec![] };
    }
    fn score(&self, x: i64, y: i64, part1: bool) -> usize {
        let mut queue = collections::VecDeque::from([(x, y)]);
        let mut score = 0;
        let mut visited = HashSet::new();
        visited.insert((x, y));
        while queue.len() > 0 {
            let cur = queue.pop_front().unwrap();
            if part1 && self.grid[cur.0 as usize][cur.1 as usize] == '9' && !visited.contains(&cur)
            {
                score += 1;
                visited.insert(cur);
                continue;
            } else if !part1 && self.grid[cur.0 as usize][cur.1 as usize] == '9' {
                score += 1;
                visited.insert(cur);
                continue;
            }
            visited.insert(cur);
            for i in [1, -1] {
                if cur.0 + i >= 0
                    && cur.0 + i < self.grid.len() as i64
                    && self.grid[(cur.0 + i) as usize][cur.1 as usize] as i64 - 1
                        == self.grid[cur.0 as usize][cur.1 as usize] as i64
                {
                    queue.push_back(((cur.0 + i), cur.1));
                }
                if cur.1 + i >= 0
                    && cur.1 + i < self.grid[0].len() as i64
                    && self.grid[cur.0 as usize][(cur.1 as i64 + i) as usize] as i64 - 1
                        == self.grid[cur.0 as usize][cur.1 as usize] as i64
                {
                    queue.push_back((cur.0, (cur.1 as i64 + i)));
                }
            }
        }
        score
    }
}

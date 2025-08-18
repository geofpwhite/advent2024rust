use std::{collections::HashSet, fs::File, io::Read};

pub(crate) fn advent6() {
    let mut contents = String::new();
    File::open("inputs/advent6.txt")
        // File::open("inputs/test6.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents = contents.replace("\r", "\n");
    let rlines = contents.split("\n").filter(|s| s.to_string() != "");
    let mut lines = rlines.clone();
    let mut gao = GuardAndObstacles::new();
    let mut start = Coords { x: 0, y: 0 };
    let size_cl = lines.count();
    lines = rlines;
    gao.max_x = size_cl;

    lines.enumerate().for_each(|(i, line)| {
        gao.max_y = line.len();
        line.char_indices().for_each(|(j, c)| match c {
            '^' => {
                gao.guard = Guard::new(i as i32, j as i32, Dir::U);
                start.x = i as i32;
                start.y = j as i32;
            }
            '#' => {
                gao.obstacles.push(Coords {
                    x: i as i32,
                    y: j as i32,
                });
            }
            _ => {}
        });
    });
    let startgao = gao.clone();
    while gao.next() {}
    let mut visited_squares: HashSet<Coords> = HashSet::new();
    gao.visited.iter().for_each(|g| {
        visited_squares.insert(g.pos);
    });
    let mut loops = 0;
    visited_squares
        .iter()
        .filter(|i| **i != start)
        .for_each(|g| {
            let mut check_gao = startgao.clone();
            check_gao.obstacles.push(*g);
            if check_gao.will_loop() {
                loops += 1;
            }
        });
    println!("{loops}");
    println!("{}", visited_squares.len());
}

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
struct Coords {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn rotate_right(&mut self) {
        *self = match self {
            Dir::U => Dir::R,
            Dir::D => Dir::L,
            Dir::R => Dir::D,
            Dir::L => Dir::U,
        }
    }
    fn rotate_left(&mut self) {
        *self = match self {
            Dir::D => Dir::R,
            Dir::U => Dir::L,
            Dir::L => Dir::D,
            Dir::R => Dir::U,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
struct Guard {
    pub dir: Dir,
    pub pos: Coords,
}
impl Guard {
    fn new(x: i32, y: i32, dir: Dir) -> Guard {
        Guard {
            dir: dir,
            pos: Coords { x: x, y: y },
        }
    }
}

#[derive(Clone)]
struct GuardAndObstacles {
    pub guard: Guard,
    pub obstacles: Vec<Coords>,
    pub visited: HashSet<Guard>,
    pub num_visited: i32,
    pub max_x: usize,
    pub max_y: usize,
}

impl GuardAndObstacles {
    fn new() -> GuardAndObstacles {
        GuardAndObstacles {
            guard: Guard {
                dir: Dir::U,
                pos: Coords { x: 0, y: 0 },
            },
            obstacles: vec![],
            visited: HashSet::new(),
            num_visited: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    // return false when exit otherwise return true
    fn next(&mut self) -> bool {
        let mut dx @ mut dy = 0 as i32;
        let mut visited = 0;
        match self.guard.dir {
            Dir::U => dx -= 1,
            Dir::D => dx += 1,
            Dir::R => dy += 1,
            Dir::L => dy -= 1,
        };
        let mut c = self.guard.clone();
        while (!self.obstacles.contains(&c.pos))
            && (c.pos.x >= 0
                && c.pos.x < self.max_x as i32
                && c.pos.y >= 0
                && c.pos.y < self.max_y as i32)
        {
            if !self.visited.contains(&c) {
                self.visited.insert(c);
                visited += 1;
            } else {
                self.guard = c;
                return false;
            }
            c.pos.x += dx;
            c.pos.y += dy;
        }
        if !self.obstacles.contains(&c.pos) {
            self.visited.remove(&c);
            self.num_visited += visited;
            self.guard = c;
            return false;
        }
        c.pos.x -= dx;
        c.pos.y -= dy;
        self.num_visited += visited;
        self.guard.pos = c.pos;
        self.guard.dir.rotate_right();
        true
    }
    fn will_loop(&mut self) -> bool {
        loop {
            if !self.next() {
                return self.visited.contains(&self.guard);
            }
        }
    }
}

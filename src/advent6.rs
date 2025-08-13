use std::{collections::HashSet, fs::File, io::Read};

pub(crate) fn advent6() {
    let mut contents = String::new();
    File::open("inputs/advent5.txt")
        // File::open("inputs/test5.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents = contents.replace("\r", "\n");
    let rlines = contents.split("\n").filter(|s| s.to_string() != "");
    let lines = rlines.clone();
    let mut gao = GuardAndObstacles::new();
    let size_cl = lines.clone().collect::<Vec<&str>>();

    gao.max_x = size_cl.len();
    gao.max_y = size_cl[0].len();

    lines.enumerate().for_each(|(i, line)| {
        line.char_indices().for_each(|(j, c)| match c {
            '^' => {
                gao.guard = Guard::new(i, j);
            }
            '#' => {
                gao.obstacles.push(Coords { x: i, y: j });
            }
            _ => {}
        });
    });
}

struct Coords {
    pub x: usize,
    pub y: usize,
}
enum Dir {
    U,
    D,
    L,
    R,
}

struct Guard {
    pub dir: Dir,
    pub pos: Coords,
}
impl Guard {
    fn new(x: usize, y: usize) -> Guard {
        Guard {
            dir: Dir::U,
            pos: Coords { x: x, y: x },
        }
    }
}

struct GuardAndObstacles {
    pub guard: Guard,
    pub obstacles: Vec<Coords>,
    pub visited: HashSet<Guard>,
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
            max_x: 0,
            max_y: 0,
        }
    }
    fn next() {}
}

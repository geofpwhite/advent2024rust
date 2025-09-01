use std::collections::{vec_deque, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::ptr::null;

pub(crate) fn advent15() {
    // Open the file
    let mut contents = String::new();
    File::open("inputs/advent15.txt")
        // File::open("inputs/test15.txt")
        // File::open("inputs/smalltest15.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents = contents.replace("\r\n", "\n");
    let mut map_and_moves = contents.split("\n\n");
    let mut position = Position::new();
    let mut big_box_position = BigBoxPosition::new();
    map_and_moves
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            line.char_indices().for_each(|(j, c)| match c {
                '#' => {
                    big_box_position.walls.insert(BigBox {
                        x: i as i64,
                        y1: j as i64 * 2,
                        y2: j as i64 * 2 + 1,
                    });
                    position.walls.insert(Coords {
                        x: i as i64,
                        y: j as i64,
                    });
                }
                'O' => {
                    big_box_position.boxes.insert(BigBox {
                        x: i as i64,
                        y1: j as i64 * 2,
                        y2: j as i64 * 2 + 1,
                    });
                    position.boxes.insert(Coords {
                        x: i as i64,
                        y: j as i64,
                    });
                }
                '@' => {
                    position.robot = Coords {
                        x: i as i64,
                        y: j as i64,
                    };
                    big_box_position.robot = Coords {
                        x: i as i64,
                        y: j as i64 * 2,
                    }
                }
                _ => {}
            });
        });
    let moves = map_and_moves.next().unwrap();
    moves.chars().for_each(|c| {
        position.move_robot(c);
        big_box_position.move_robot(c);
    });
    let mut sum = 0;
    let mut sum2 = 0;
    position.boxes.iter().for_each(|coords| {
        sum += (coords.x * 100) + coords.y;
    });
    big_box_position.boxes.iter().for_each(|coords| {
        sum2 += (coords.x * 100) + coords.y1;
    });
    println!("{sum}");
    println!("{sum2}");
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coords {
    x: i64,
    y: i64,
}
impl Coords {
    fn add(self, other: Coords) -> Self {
        Coords {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
struct Position {
    robot: Coords,
    walls: HashSet<Coords>,
    boxes: HashSet<Coords>,
}
impl Position {
    fn new() -> Self {
        Position {
            robot: Coords { x: 0, y: 0 },
            walls: HashSet::new(),
            boxes: HashSet::new(),
        }
    }
    fn move_robot(&mut self, c: char) {
        let mut new_robot_dxdy = Coords { x: 0, y: 0 };
        match c {
            'v' => {
                new_robot_dxdy.x += 1;
            }
            '>' => {
                new_robot_dxdy.y += 1;
            }
            '<' => {
                new_robot_dxdy.y -= 1;
            }
            '^' => {
                new_robot_dxdy.x -= 1;
            }
            _ => {}
        }
        self.move_boxes(new_robot_dxdy);
    }
    fn move_boxes(&mut self, dxdy: Coords) {
        let mut cur = self.robot.add(dxdy);
        while self.boxes.contains(&cur) {
            cur = cur.add(dxdy);
        }
        if self.walls.contains(&cur) {
            return;
        }
        self.boxes.insert(cur);
        self.boxes.remove(&self.robot.add(dxdy));
        self.robot = self.robot.add(dxdy);
    }
}

struct BigBoxPosition {
    robot: Coords,
    walls: HashSet<BigBox>,
    boxes: HashSet<BigBox>,
}
impl BigBoxPosition {
    fn new() -> Self {
        BigBoxPosition {
            robot: Coords { x: 0, y: 0 },
            walls: HashSet::new(),
            boxes: HashSet::new(),
        }
    }
    fn move_robot(&mut self, c: char) {
        let mut new_robot_dxdy = Coords { x: 0, y: 0 };
        match c {
            'v' => {
                new_robot_dxdy.x += 1;
            }
            '>' => {
                new_robot_dxdy.y += 1;
            }
            '<' => {
                new_robot_dxdy.y -= 1;
            }
            '^' => {
                new_robot_dxdy.x -= 1;
            }
            _ => {}
        }
        if new_robot_dxdy.y != 0 {
            let (unblocked, connected) =
                self.connected_boxes_that_can_be_moved_horizontal(new_robot_dxdy.y);
            if unblocked {
                self.move_boxes(new_robot_dxdy, connected);
            }
        } else if new_robot_dxdy.x != 0 {
            let (unblocked, connected) =
                self.connected_boxes_that_can_be_moved_vertical(new_robot_dxdy.x);
            if unblocked {
                self.move_boxes(new_robot_dxdy, connected);
            }
        }
    }
    fn move_boxes(&mut self, dxdy: Coords, boxes_to_move: HashSet<BigBox>) {
        self.robot = self.robot.add(dxdy);
        let mut to_insert = HashSet::new();
        for big_box in boxes_to_move.iter() {
            let mut bb = big_box.clone();
            if !self.boxes.contains(big_box) {
                println!("{} {} {}", big_box.x, big_box.y1, big_box.y2);
                continue;
            }
            self.boxes.remove(big_box);
            bb.x += dxdy.x;
            bb.y1 += dxdy.y;
            bb.y2 += dxdy.y;
            to_insert.insert(bb);
        }
        for bb in to_insert {
            self.boxes.insert(bb);
        }
    }

    fn connected_boxes_that_can_be_moved_horizontal(&mut self, dy: i64) -> (bool, HashSet<BigBox>) {
        let mut hs = HashSet::new();
        let mut check = self.robot.clone();
        let mut unblocked = true;
        check.y += dy;
        if dy == 1 {
            while self.boxes.contains(&BigBox {
                x: check.x,
                y1: check.y,
                y2: check.y + 1,
            }) {
                hs.insert(BigBox {
                    x: check.x,
                    y1: check.y,
                    y2: check.y + 1,
                });
                check.y += 2;
            }
            if self.walls.contains(&BigBox {
                x: check.x,
                y1: check.y,
                y2: check.y + 1,
            }) {
                hs = HashSet::new();
                unblocked = false;
            }
        } else {
            while self.boxes.contains(&BigBox {
                x: check.x,
                y1: check.y - 1,
                y2: check.y,
            }) {
                hs.insert(BigBox {
                    x: check.x,
                    y1: check.y - 1,
                    y2: check.y,
                });
                check.y -= 2;
            }
            if self.walls.contains(&BigBox {
                x: check.x,
                y1: check.y - 1,
                y2: check.y,
            }) {
                hs = HashSet::new();
                unblocked = false;
            }
        }

        (unblocked, hs)
    }
    fn connected_boxes_that_can_be_moved_vertical(&mut self, dx: i64) -> (bool, HashSet<BigBox>) {
        let mut hs = HashSet::new();
        let check = self.robot.clone();
        let mut unblocked = true;
        let mut queue = vec_deque::VecDeque::from([check]);
        while queue.len() > 0 {
            let cur = queue.pop_front().unwrap();

            let to_check = BigBox {
                x: cur.x + dx,
                y1: cur.y,
                y2: cur.y + 1,
            };
            let to_check2 = BigBox {
                x: cur.x + dx,
                y1: cur.y - 1,
                y2: cur.y,
            };
            if self.walls.contains(&to_check) || self.walls.contains(&to_check2) {
                unblocked = false;
                break;
            }

            if self.boxes.contains(&to_check) {
                hs.insert(to_check);
                queue.push_back(Coords {
                    x: cur.x + dx,
                    y: cur.y,
                });
                queue.push_back(Coords {
                    x: cur.x + dx,
                    y: cur.y + 1,
                });
            } else if self.boxes.contains(&to_check2) {
                hs.insert(to_check2);
                queue.push_back(Coords {
                    x: cur.x + dx,
                    y: cur.y - 1,
                });
                queue.push_back(Coords {
                    x: cur.x + dx,
                    y: cur.y,
                });
            }
        }

        (unblocked, hs)
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct BigBox {
    x: i64,
    y1: i64,
    y2: i64,
}

use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
    usize,
};

pub(crate) fn advent20() {
    let mut contents = String::new();
    File::open("inputs/advent20.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    let lines = contents.lines();
    let max_x = lines.clone().count();
    let mut max_y = 0;
    let mut nm = NodeMap::new();
    let mut start = Coords { x: 0, y: 0 };
    let mut end = Coords { x: 0, y: 0 };
    lines.enumerate().for_each(|(i, line)| {
        if max_y == 0 {
            max_y = line.len()
        }
        line.char_indices().for_each(|(j, char)| match char {
            'S' | 'E' | '.' => {
                let c = Coords {
                    x: i as i16,
                    y: j as i16,
                };
                nm.insert(c, Node::new(c));
                if char == 'S' {
                    start = c;
                }
                if char == 'E' {
                    end = c;
                }
            }
            _ => {}
        });
    });
    let xm = nm.clone();
    for (c, n) in nm.iter_mut() {
        let udlr = [
            Coords { x: c.x - 1, y: c.y },
            Coords { x: c.x + 1, y: c.y },
            Coords { x: c.x, y: c.y - 1 },
            Coords { x: c.x, y: c.y + 1 },
        ];
        for (i, neighbor_coords) in udlr.iter().enumerate() {
            if xm.contains_key(&neighbor_coords) {
                match i {
                    0 => n.up = Some(*neighbor_coords),
                    1 => n.down = Some(*neighbor_coords),
                    2 => n.left = Some(*neighbor_coords),
                    3 => n.right = Some(*neighbor_coords),
                    _ => {}
                }
            }
        }
    }
    let (steps_to_beat, path) = nm.search(start, end);
    let mut count = 0;
    (0..max_x).for_each(|x| {
        (0..max_y).for_each(|y| {
            let c = Coords {
                x: x as i16,
                y: y as i16,
            };
            if !nm.contains_key(&c) {
                let mut nm = nm.clone();
                nm.insert(
                    c,
                    Node {
                        coords: c,
                        up: None,
                        down: None,
                        left: None,
                        right: None,
                    },
                );
                let n = nm.get_mut(&c).unwrap();
                let udlr = [
                    Coords { x: c.x - 1, y: c.y },
                    Coords { x: c.x + 1, y: c.y },
                    Coords { x: c.x, y: c.y - 1 },
                    Coords { x: c.x, y: c.y + 1 },
                ];
                for (i, neighbor_coords) in udlr.iter().enumerate() {
                    if xm.contains_key(&neighbor_coords) {
                        match i {
                            0 => n.up = Some(*neighbor_coords),
                            1 => n.down = Some(*neighbor_coords),
                            2 => n.left = Some(*neighbor_coords),
                            3 => n.right = Some(*neighbor_coords),
                            _ => {}
                        }
                    }
                }
                for (i, neighbor_coords) in udlr.iter().enumerate() {
                    if let Some(ne) = nm.get_mut(neighbor_coords) {
                        match i {
                            0 => ne.down = Some(c),
                            1 => ne.up = Some(c),
                            2 => ne.right = Some(c),
                            3 => ne.left = Some(c),
                            _ => {}
                        }
                    }
                }

                let (steps, path) = nm.search(start, end);
                if steps_to_beat - steps > 99 {
                    count += 1;
                }
            }
        });
    });
    println!("{count}");
    count = 0;
    path.iter().enumerate().for_each(|(i, c)| {
        count += path
            .iter()
            .skip(i + 100)
            .filter(|c2| c2.distance(c) <= 20)
            .count();
    });
    println!("{count}");
}

type NodeMap = HashMap<Coords, Node>;

trait Searcher {
    fn search(&mut self, start: Coords, end: Coords) -> (usize, Vec<Coords>);
}
impl Searcher for NodeMap {
    fn search(&mut self, start: Coords, end: Coords) -> (usize, Vec<Coords>) {
        if !self.contains_key(&start) || !self.contains_key(&end) {
            return (usize::MAX, vec![]);
        }
        let mut visited = HashMap::new();
        let begin = QueueNode::new(start, 0, vec![]);
        let mut queue = Queue::from([begin]);
        let mut end_path = vec![];
        while queue.len() > 0 {
            let mut cur = queue.pop_front().unwrap();
            let node = self.get(&cur.coords).unwrap();
            if cur.coords == end {
                end_path = cur.prev.clone();
            }
            visited.insert(cur.coords, cur.steps);
            [node.up, node.down, node.left, node.right]
                .iter()
                .for_each(|neighbor| {
                    if let Some(neighbor) = neighbor {
                        let steps = visited.get(neighbor).unwrap_or(&usize::MAX);
                        if cur.steps + 1 < *steps {
                            cur.prev.push(*neighbor);
                            queue.push_back(QueueNode {
                                coords: *neighbor,
                                steps: cur.steps + 1,
                                prev: cur.prev.clone(),
                            });
                        }
                    }
                });
        }
        (*visited.get(&end).unwrap_or(&usize::MAX), end_path)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Coords {
    x: i16,
    y: i16,
}
impl Coords {
    fn distance(self, c: &Coords) -> usize {
        ((self.x - c.x).abs() + (self.y - c.y).abs()) as usize
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Node {
    coords: Coords,
    up: Option<Coords>,
    down: Option<Coords>,
    left: Option<Coords>,
    right: Option<Coords>,
}

impl Node {
    fn new(c: Coords) -> Self {
        Self {
            coords: c,
            up: None,
            down: None,
            left: None,
            right: None,
        }
    }
}

struct QueueNode {
    coords: Coords,
    steps: usize,
    prev: Vec<Coords>,
}

impl QueueNode {
    fn new(coords: Coords, steps: usize, prev: Vec<Coords>) -> Self {
        Self {
            coords: coords,
            steps: steps,
            prev: prev,
        }
    }
}

type Queue = VecDeque<QueueNode>;

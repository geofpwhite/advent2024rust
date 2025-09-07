use std::{
    arch::x86_64,
    boxed,
    collections::{vec_deque, HashMap, HashSet},
    fs::File,
    io::Read,
};

pub(crate) fn advent16() {
    let mut contents = String::new();
    File::open("inputs/advent16.txt")
        // File::open("inputs/test16.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    let mut tile_map = HashSet::new();
    let mut parse_node_map: HashMap<Coords, ParseNode> = HashMap::new();
    let mut real_parse_node_map: HashMap<Coords, ParseNode> = HashMap::new();
    let mut node_map: HashMap<Coords, Node> = HashMap::new();
    let mut start = Coords { x: 0, y: 0 };
    let mut end = Coords { x: 0, y: 0 };
    contents.lines().enumerate().for_each(|(i, line)| {
        line.char_indices().for_each(|(j, c)| {
            if c == '.' || c == 'E' || c == 'S' {
                tile_map.insert(Coords { x: i, y: j });
            }
            if c == 'E' {
                end = Coords { x: i, y: j };
            }
            if c == 'S' {
                start.x = i;
                start.y = j;
            }
        });
    });
    tile_map.iter().for_each(|coords| {
        let coords = coords;
        let pn = ParseNode::new(coords.x, coords.y);
        parse_node_map.insert(*coords, pn);
    });
    parse_node_map.iter().for_each(|(c, pnode)| {
        let mut p = pnode.clone();
        if c.x > 0 {
            if parse_node_map.contains_key(&Coords { x: c.x - 1, y: c.y }) {
                p.u = Some(Coords { x: c.x - 1, y: c.y });
            }
        }
        if parse_node_map.contains_key(&Coords { x: c.x + 1, y: c.y }) {
            p.d = Some(Coords { x: c.x + 1, y: c.y });
        }
        if c.y > 0 {
            if parse_node_map.contains_key(&Coords { x: c.x, y: c.y - 1 }) {
                p.r = Some(Coords { x: c.x, y: c.y - 1 });
            }
        }
        if parse_node_map.contains_key(&Coords { x: c.x, y: c.y + 1 }) {
            p.l = Some(Coords { x: c.x, y: c.y + 1 });
        }
        real_parse_node_map.insert(*c, p);
    });

    let start_node = real_parse_node_map.get(&start).unwrap().clone();
    let startq_node = QueueNode {
        node: start_node,
        distance: 0,
        direction: Direction::R,
        path: Box::from(vec![]),
    };
    let (score, paths) = search(&real_parse_node_map, startq_node, end);
    println!("{score}");
    println!("{}", paths.len());
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coords {
    x: usize,
    y: usize,
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Node {
    x: usize,
    y: usize,
    u: Option<(Coords, usize)>,
    d: Option<(Coords, usize)>,
    r: Option<(Coords, usize)>,
    l: Option<(Coords, usize)>,
}
impl Node {
    fn new(pn: ParseNode) -> Self {
        Self {
            x: pn.x,
            y: pn.y,
            u: None,
            d: None,
            r: None,
            l: None,
        }
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct ParseNode {
    x: usize,
    y: usize,
    u: Option<Coords>,
    d: Option<Coords>,
    r: Option<Coords>,
    l: Option<Coords>,
}
impl ParseNode {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x,
            y: y,
            u: None,
            d: None,
            r: None,
            l: None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct QueueNode {
    node: ParseNode,
    distance: usize,
    direction: Direction,
    path: Box<Vec<Coords>>,
}

fn search(
    nodes: &HashMap<Coords, ParseNode>,
    start: QueueNode,
    end: Coords,
) -> (usize, HashSet<Coords>) {
    let mut min = usize::MAX;
    let mut visited: HashMap<Coords, HashMap<Direction, usize>> = HashMap::new();
    let mut q = vec_deque::VecDeque::new();
    let mut winning_paths = HashSet::new();
    q.push_back(start);
    while q.len() > 0 {
        let cur = q.pop_front().unwrap();
        // println!("{cur:?}");
        let cur_coords = &Coords {
            x: cur.node.x,
            y: cur.node.y,
        };
        if cur_coords.x == end.x && cur_coords.y == end.y {
            if cur.distance < min {
                min = cur.distance;
                winning_paths = HashSet::new();
                winning_paths.insert(end);
                for c in cur.path.iter() {
                    winning_paths.insert(*c);
                }
            } else if cur.distance == min {
                for c in cur.path.iter() {
                    winning_paths.insert(*c);
                }
            }
        }
        if visited.contains_key(&Coords {
            x: cur.node.x,
            y: cur.node.y,
        }) {
            let mut hm = visited
                .get(&Coords {
                    x: cur.node.x,
                    y: cur.node.y,
                })
                .unwrap()
                .clone();
            let dist = hm.get(&cur.direction).unwrap_or(&usize::MAX);
            if dist < &cur.distance {
                continue;
            }
            hm.insert(cur.direction, cur.distance);

            visited.insert(
                Coords {
                    x: cur.node.x,
                    y: cur.node.y,
                },
                hm.clone(),
            );
        } else {
            let mut hm = HashMap::new();
            hm.insert(cur.direction, cur.distance);
            visited.insert(
                Coords {
                    x: cur.node.x,
                    y: cur.node.y,
                },
                hm,
            );
        }

        let path = &cur.path;
        let mut new_path = path.clone();
        new_path.push(Coords {
            x: cur.node.x,
            y: cur.node.y,
        });
        if let Some(neighbor) = cur.node.u {
            let mut possible = QueueNode {
                node: *nodes.get(&neighbor).unwrap(),
                distance: cur.distance,
                direction: Direction::U,
                path: new_path.clone(),
            };
            match cur.direction {
                Direction::L | Direction::R => {
                    possible.distance += 1001;
                    q.push_back(possible);
                }
                Direction::U => {
                    possible.distance += 1;
                    q.push_back(possible);
                }
                Direction::D => {}
            }
        }
        if let Some(neighbor) = cur.node.d {
            let mut possible = QueueNode {
                node: *nodes.get(&neighbor).unwrap(),
                distance: cur.distance,
                direction: Direction::D,
                path: new_path.clone(),
            };
            match cur.direction {
                Direction::L | Direction::R => {
                    possible.distance += 1001;
                    q.push_back(possible);
                }
                Direction::D => {
                    possible.distance += 1;
                    q.push_back(possible);
                }
                Direction::U => {}
            }
        }
        if let Some(neighbor) = cur.node.r {
            let mut possible = QueueNode {
                node: *nodes.get(&neighbor).unwrap(),
                distance: cur.distance,
                direction: Direction::R,
                path: new_path.clone(),
            };
            match cur.direction {
                Direction::U | Direction::D => {
                    possible.distance += 1001;
                    q.push_back(possible);
                }
                Direction::R => {
                    possible.distance += 1;
                    q.push_back(possible);
                }
                Direction::L => {}
            }
        }
        if let Some(neighbor) = cur.node.l {
            let mut possible = QueueNode {
                node: *nodes.get(&neighbor).unwrap(),
                distance: cur.distance,
                direction: Direction::L,
                path: new_path.clone(),
            };
            match cur.direction {
                Direction::U | Direction::D => {
                    possible.distance += 1001;
                    q.push_back(possible);
                }
                Direction::L => {
                    possible.distance += 1;
                    q.push_back(possible);
                }
                Direction::R => {}
            }
        }
    }

    (min, winning_paths)
}

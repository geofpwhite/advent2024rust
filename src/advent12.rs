use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

pub(crate) fn advent12() {
    let mut contents: String = String::new();
    File::open("inputs/advent12.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    let mut g = Grid::new();
    contents.split("\n").enumerate().for_each(|(i, line)| {
        line.trim().char_indices().for_each(|(j, c)| {
            g.insert((i, j), Node::new(c));
        });
    });
    println!("{contents}");
    for (i, line) in contents.split("\n").enumerate() {
        for (j, _) in line.char_indices() {
            let has_up = g.contains_key(&((i as i64 - 1) as usize, j));
            let has_down = g.contains_key(&(i + 1, j));
            let has_left = g.contains_key(&(i, (j as i64 - 1) as usize));
            let has_right = g.contains_key(&(i, j + 1));
            let n = g.get_mut(&(i, j.clone())).unwrap();
            match has_up {
                true => (*n).up = Some((i - 1, j)),
                false => (*n).up = None,
            }
            match has_right {
                true => (*n).right = Some((i, j + 1)),
                false => (*n).right = None,
            }
            match has_left {
                true => (*n).left = Some((i, j - 1)),
                false => (*n).left = None,
            }
            match has_down {
                true => (*n).down = Some((i + 1, j)),
                false => (*n).down = None,
            }
        }
    }
    // println!("{:?}", g.iter());
    println!("{}", g.score());
    println!("{}", g.score2());
}
type Coords = (usize, usize);

type Grid = HashMap<Coords, Node>;
trait GridTrait {
    fn score2(&mut self) -> usize;
    fn score(&mut self) -> usize;
    fn other_shape_coords(&mut self, coords: Coords) -> (HashSet<Coords>, usize);
    fn get_sides(&mut self, hset: &HashSet<Coords>) -> usize;
}

impl GridTrait for Grid {
    fn other_shape_coords(&mut self, coords: Coords) -> (HashSet<Coords>, usize) {
        let mut hs = HashSet::from([coords]);
        let mut queue = vec![coords];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut perim = 0;
        visited.insert(coords);
        while queue.len() > 0 {
            let cur = self.get(&queue[0]).unwrap();
            hs.insert(queue[0]);
            queue = queue.iter().skip(1).map(|node| *node).collect();
            let neighbors = [cur.up, cur.down, cur.left, cur.right];
            for neighbor in neighbors {
                if let Some(n) = neighbor {
                    let node = self.get(&n).unwrap();
                    if !visited.contains(&n) && node.value == cur.value {
                        queue.push(n);
                        visited.insert(n);
                    } else if node.value != cur.value {
                        perim += 1;
                    }
                } else {
                    perim += 1;
                }
            }
        }
        (hs, perim)
    }
    fn score(&mut self) -> usize {
        let mut score = 0;
        let mut visited: HashSet<Coords> = HashSet::new();
        let maxx = self.iter().max_by(|x, y| x.0 .0.cmp(&y.0 .0)).unwrap().0 .0 + 1;
        let maxy = self.iter().max_by(|x, y| x.0 .1.cmp(&y.0 .1)).unwrap().0 .1 + 1;
        println!("{maxx} {maxy}");
        for i in 0..maxx {
            for j in 0..maxy {
                if visited.contains(&(i, j)) {
                    continue;
                }
                let (shape, perim) = self.other_shape_coords((i, j));
                score += shape.len() * perim;
                shape.iter().for_each(|c| {
                    visited.insert(*c);
                });
            }
        }
        score
    }
    fn score2(&mut self) -> usize {
        let mut score = 0;
        let mut visited: HashSet<Coords> = HashSet::new();
        let maxx = self.iter().max_by(|x, y| x.0 .0.cmp(&y.0 .0)).unwrap().0 .0 + 1;
        let maxy = self.iter().max_by(|x, y| x.0 .1.cmp(&y.0 .1)).unwrap().0 .1 + 1;
        for i in 0..maxx {
            for j in 0..maxy {
                if visited.contains(&(i, j)) {
                    continue;
                }
                let (shape, _) = self.other_shape_coords((i, j));
                let sides = self.get_sides(&shape);
                score += shape.len() * sides;
                shape.iter().for_each(|c| {
                    visited.insert(*c);
                });
            }
        }
        score
    }
    fn get_sides(&mut self, shape: &HashSet<Coords>) -> usize {
        let mut corners: HashSet<(usize, usize)> = HashSet::new();
        let mut num = 0;
        shape.iter().for_each(|coords| {
            let mut c0 = coords.0;
            let mut c1 = coords.1;
            if coords.0 == 0 {
                c0 = usize::MAX;
            } else {
                c0 -= 1;
            }
            if coords.1 == 0 {
                c1 = usize::MAX;
            } else {
                c1 -= 1;
            }
            corners.insert((coords.0, coords.1));
            corners.insert((c0, coords.1));
            corners.insert((coords.0, c1));
            corners.insert((c0, c1));
            if coords.1 == 0 {
                corners.insert((coords.0, usize::MAX));
            } else {
                corners.insert((coords.0, coords.1 - 1));
            }
        });
        corners.iter().for_each(|coords| {
            //ul, ur, ld, rd
            let mut c0 = coords.0;
            if coords.0 == usize::MAX {
                c0 = 0;
            } else {
                c0 += 1;
            }
            let mut c1 = coords.1;
            if coords.1 == usize::MAX {
                c1 = 0;
            } else {
                c1 += 1;
            }
            let square_coords: [Coords; 4] = [
                (coords.0, coords.1),
                (c0, coords.1),
                (coords.0, c1),
                (c0, c1),
            ];
            let mut good = vec![];
            for i in 0..4 {
                match shape.get(&square_coords[i]) {
                    Some(_) => {
                        good.push(square_coords[i]);
                    }
                    None => (),
                };
            }
            if (good.len() % 2) == 1 {
                num += 1;
            }
            if good.len() == 2 {
                if good[0].0 != good[1].0 && good[0].1 != good[1].1 {
                    num += 2;
                }
            }
            // println!("{good:?}");
        });
        num
    }
}

#[derive(Debug)]
struct Node {
    value: char,
    up: Option<Coords>,
    down: Option<Coords>,
    left: Option<Coords>,
    right: Option<Coords>,
}
impl Node {
    fn new(value: char) -> Self {
        Node {
            value: (value),
            up: None,
            down: None,
            left: None,
            right: None,
        }
    }
}

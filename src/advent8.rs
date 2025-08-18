use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

pub(crate) fn advent8() {
    let mut contents = String::new();
    File::open("inputs/advent8.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents = contents.replace("\r", "\n");
    let lines = contents.split("\n").filter(|s| s.to_string() != "");
    let max_y = lines.clone().next().unwrap().len();
    let max_x = lines.clone().count();
    let mut nmap = NodeMap::new(max_x, max_y);
    lines.enumerate().for_each(|(i, s)| {
        s.char_indices().for_each(|(j, c)| match c {
            '.' => return,
            _ => {
                nmap.insert(&c, [i as i32, j as i32]);
            }
        });
    });
    nmap.create_antinodes();
    let mut sol = HashSet::new();
    nmap.antinodes.iter().for_each(|(_, k)| {
        k.iter()
            .filter(|v| {
                v[0] < nmap.max_x as i32 && v[1] < nmap.max_y as i32 && v[0] >= 0 && v[1] >= 0
            })
            .for_each(|v| {
                sol.insert(v);
            });
    });
    println!("{}", sol.len());
    nmap.create_antinodes_2();
    let mut sol = HashSet::new();
    nmap.antinodes.iter().for_each(|(_, k)| {
        k.iter()
            .filter(|v| {
                v[0] < nmap.max_x as i32 && v[1] < nmap.max_y as i32 && v[0] >= 0 && v[1] >= 0
            })
            .for_each(|v| {
                sol.insert(v);
            });
    });
    println!("{}", sol.len())
}

type Coords = [i32; 2];

#[derive(Debug)]
struct NodeMap {
    nodes: HashMap<char, Vec<Coords>>,
    antinodes: HashMap<char, HashSet<Coords>>,
    max_x: usize,
    max_y: usize,
}

impl NodeMap {
    fn new(x: usize, y: usize) -> Self {
        NodeMap {
            nodes: HashMap::new(),
            antinodes: HashMap::new(),
            max_x: x,
            max_y: y,
        }
    }
    fn insert(&mut self, k: &char, v: [i32; 2]) {
        if self.nodes.contains_key(&k) {
            self.nodes.get_mut(k).unwrap().push(v);
        } else {
            self.nodes.insert(*k, vec![v]);
        }
    }
    fn create_antinodes(&mut self) {
        self.nodes.iter().enumerate().for_each(|(_, (char, v))| {
            v.iter().enumerate().for_each(|(i, c)| {
                let mut vc = v.clone();
                vc.remove(i);
                vc.iter().for_each(|c1| {
                    let dx = c1[0] - c[0];
                    let dy = c1[1] - c[1];
                    let check = [[c1[0] + dx, c1[1] + dy], [c[0] - dx, c[1] - dy]];
                    if self.antinodes.contains_key(char) {
                        let ans = self.antinodes.get_mut(char).unwrap();
                        ans.insert(check[0]);
                        ans.insert(check[1]);
                    } else {
                        self.antinodes.insert(*char, HashSet::from(check));
                    }
                });
            });
        });
    }
    fn create_antinodes_2(&mut self) {
        self.antinodes = HashMap::new();
        self.nodes.iter().enumerate().for_each(|(_, (char, v))| {
            v.iter().enumerate().for_each(|(i, c)| {
                let mut vc = v.clone();
                vc.remove(i);
                vc.iter().for_each(|c1| {
                    let dx = c1[0] - c[0];
                    let dy = c1[1] - c[1];
                    let mut check = vec![[c1[0] + dx, c1[1] + dy], [c[0] - dx, c[1] - dy]];
                    let mut cur_x = c1[0];
                    let mut cur_y = c1[1];
                    while cur_x >= 0
                        && cur_x < self.max_x as i32
                        && cur_y >= 0
                        && cur_y < self.max_y as i32
                    {
                        cur_x += dx;
                        cur_y += dy;
                        check.push([cur_x, cur_y])
                    }
                    let mut cur_x = c1[0];
                    let mut cur_y = c1[1];
                    while cur_x >= 0
                        && cur_x < self.max_x as i32
                        && cur_y >= 0
                        && cur_y < self.max_y as i32
                    {
                        cur_x -= dx;
                        cur_y -= dy;
                        if [cur_x, cur_y] != *c1 {
                            check.push([cur_x, cur_y])
                        }
                    }
                    if !self.antinodes.contains_key(char) {
                        self.antinodes.insert(*char, HashSet::new());
                    }
                    let ans = self.antinodes.get_mut(char).unwrap();
                    check.iter().for_each(|f| {
                        ans.insert(*f);
                    });
                });
            });
        });
    }
}

use std::{arch::x86_64, f32::INFINITY, fs::File, io::Read, usize};

pub(crate) fn advent9() {
    let mut contents = String::new();
    let mut c = Code::new();
    let mut parse_index = 0;
    File::open("inputs/advent9.txt")
        // File::open("inputs/test9.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents = contents
        .replace("\r", "")
        .replace("\n", "")
        .trim()
        .to_string();
    contents.char_indices().for_each(|(i, val)| {
        match (i as i32) % 2 {
            0 => {
                c.values.push((
                    i as i32 / 2,
                    [parse_index, parse_index + val.to_digit(10).unwrap() as i32],
                ));
                parse_index += val.to_digit(10).unwrap() as i32;
            }
            1 | _ => {
                c.gaps
                    .push([parse_index, parse_index + val.to_digit(10).unwrap() as i32]);
                parse_index += val.to_digit(10).unwrap() as i32;
            }
        };
    });
    let hold = c.clone();
    c.compact();
    let mut sum = 0 as i64;
    c.values.iter().for_each(|(value, coords)| {
        for i in coords[0]..coords[1] {
            sum += (value * i) as i64;
        }
    });
    println!("{sum}");
    c = hold;
    c.compact2();
    sum = 0;
    c.values.iter().for_each(|(value, coords)| {
        for i in coords[0]..coords[1] {
            sum += (*value as i64 * i as i64) as i64;
        }
    });
    println!("{sum}");
}

type Range = [i32; 2];

#[derive(Clone)]
struct Code {
    gaps: Vec<Range>,
    values: Vec<(i32, Range)>,
}

impl Code {
    fn new() -> Self {
        Self {
            gaps: vec![],
            values: vec![],
        }
    }
    fn compact(&mut self) {
        let mut rev_vals = self.values.clone();
        rev_vals.reverse();
        let mut new_values = vec![];
        loop {
            if self.gaps.len() == 0 {
                let last_element = self.values.len() - 1;
                self.values[last_element] = rev_vals[0];
                self.values.append(&mut new_values);
                break;
            }
            if self.gaps[0][0] >= self.gaps[0][1] {
                self.gaps = self.gaps.iter().skip(1).map(|ary| *ary).collect();
                continue;
            }
            let val_len = rev_vals[0].1[1] - rev_vals[0].1[0];
            let gap_len = self.gaps[0][1] - self.gaps[0][0];
            if self.gaps[0][0] >= rev_vals[0].1[0] {
                let last_element = self.values.len() - 1;
                self.values[last_element] = rev_vals[0];
                self.values.append(&mut new_values);
                break;
            }
            if val_len > gap_len {
                let new_value = (rev_vals[0].0, self.gaps[0]);
                rev_vals[0].1[1] = rev_vals[0].1[1] - gap_len;
                self.gaps = self.gaps.iter().skip(1).map(|ary| *ary).collect();
                new_values.push(new_value);
            } else if val_len < gap_len {
                let new_value = (rev_vals[0].0, [self.gaps[0][0], self.gaps[0][0] + val_len]);
                new_values.push(new_value);
                rev_vals = rev_vals.iter().skip(1).map(|ary| *ary).collect();
                self.gaps[0][0] += val_len;
                self.values.pop();
            } else {
                let new_value = (rev_vals[0].0, self.gaps[0]);
                new_values.push(new_value);
                rev_vals = rev_vals.iter().skip(1).map(|ary| *ary).collect();
                self.gaps = self.gaps.iter().skip(1).map(|ary| *ary).collect();
                self.values.pop();
            }
        }
    }
    fn compact2(&mut self) {
        self.values
            .clone()
            .iter()
            .enumerate()
            .rev()
            .for_each(|(index, (_, range))| {
                let mut to_remove = usize::MAX;
                for (u, g) in self.gaps.clone().iter().enumerate() {
                    if g[0] > range[0] {
                        break;
                    }
                    if g[1] - g[0] >= range[1] - range[0] {
                        self.values[index].1 = [g[0], g[0] + range[1] - range[0]];
                        self.gaps[u][0] += range[1] - range[0];
                        if self.gaps[u][1] - self.gaps[u][0] <= 0 {
                            to_remove = u;
                        }
                        break;
                    }
                }
                if to_remove != usize::MAX {
                    self.gaps.remove(to_remove);
                }
            })
    }
}

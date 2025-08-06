use std::{fs::File, io::Read};

pub(crate) fn advent4() {
    let mut contents = String::new();
    // File::open("inputs/test4.txt")
    File::open("inputs/advent4.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents = contents.replace("\r", "\n");
    let lines = contents.split("\n").filter(|s| s.to_string() != "");
    let mut v: Vec<Vec<char>> = vec![];
    let mut sum = 0;
    lines.for_each(|s| {
        let mut line_ary = vec![];
        s.chars().for_each(|ch| {
            line_ary.push(ch);
        });
        v.push(line_ary.clone());
    });
    v.iter().enumerate().for_each(|i| {
        i.1.iter().enumerate().for_each(|j| {
            // let v = v.clone();
            sum += search(&v, i.0 as i64, j.0 as i64);
        });
    });
    println!("{sum}");
}
fn search(lines: &Vec<Vec<char>>, i: i64, j: i64) -> usize {
    let mut sum = 0;
    // println!("{:?}",lines);
    for i1 in (-1)..2 {
        'outer: for j1 in (-1)..2 {
            if j1 == i1 && i1 == 0 {
                continue;
            }
            for (index, char) in "XMAS".char_indices() {
                let index = index as i64;
                let checksi = i + (i1 * index);
                let checksj = j + (j1 * index);
                if checksi < 0
                    || checksj < 0
                    || checksi >= lines.len() as i64
                    || checksj >= lines[0].len() as i64
                {
                    continue 'outer;
                }
                let char2 = lines[checksi as usize][checksj as usize];
                // println!("{checksi} {checksj}");
                if char2 != char {
                    continue 'outer;
                }
            }
            sum += 1;
        }
    }
    sum
}

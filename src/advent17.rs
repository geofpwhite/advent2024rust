use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

pub(crate) fn advent17() {
    let mut contents = String::new();
    File::open("inputs/advent17.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    let mut lines = contents.lines();
    let a = str::parse::<i64>(&lines.next().unwrap().replace("Register A: ", "").trim()).unwrap();
    let b = str::parse::<i64>(&lines.next().unwrap().replace("Register B: ", "").trim()).unwrap();
    let c = str::parse::<i64>(&lines.next().unwrap().replace("Register C: ", "").trim()).unwrap();
    let commands: Vec<i8> = lines
        .skip(1)
        .next()
        .unwrap()
        .replace("Program: ", "")
        .trim()
        .split(",")
        .map(str::parse::<i8>)
        .map(Result::unwrap)
        .collect();
    let mut p = Program::new(a, b, c, &commands);
    while p.next() {}
    println!("{:?} ", p.output);
    // println!("{:?}", p.output)
    let mut nums = find(0, commands.len() - 1, &commands, i64::MAX);
    println!("{}", nums);
}

fn find(input: i64, index: usize, commands: &Vec<i8>, mut min: i64) -> i64 {
    for i in 0..8 {
        let new_input = (input << 3) + i;
        let mut p = Program::new(new_input, 0, 0, commands);
        while p.next() {}
        if index == 0 {
            if p.output[0] == commands[0] as i64 {
                if new_input < min {
                    min = new_input;
                }
            }
        } else if commands[index] as i64 == p.output[0] {
            min = find(new_input, index - 1, commands, min);
        }
    }
    min
}

#[derive(Clone)]
struct Program {
    instr_pointer: usize,
    a: i64,
    b: i64,
    c: i64,
    commands: Vec<i8>,
    output: Vec<i64>,
}
impl Program {
    fn new(a: i64, b: i64, c: i64, commands: &Vec<i8>) -> Self {
        Self {
            instr_pointer: 0,
            a: a,
            b: b,
            c: c,
            commands: commands.clone(),
            output: vec![],
        }
    }
    fn next(&mut self) -> bool {
        let mut next_offset = 2;
        let instr = self.commands.get(self.instr_pointer).unwrap();
        let operand = self.commands.get(self.instr_pointer + 1).unwrap();
        let combo_operand = match operand {
            n @ 0_i8..=3_i8 => *n as i64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => {
                panic!()
            }
        };
        match instr {
            0 => {
                self.a /= 2_i64.pow(combo_operand as u32);
            }
            1 => {
                self.b ^= *operand as i64;
            }
            2 => {
                self.b = combo_operand % 8;
            }
            3 => match self.a {
                0 => {}
                n if n != 0 => {
                    self.instr_pointer = *operand as usize;
                    next_offset = 0;
                }
                _ => {}
            },
            4 => {
                self.b ^= self.c;
            }
            5 => self.output.push(combo_operand % 8),
            6 => {
                self.b = self.a / 2_i64.pow(combo_operand as u32);
            }
            7 => {
                self.c = self.a / 2_i64.pow(combo_operand as u32);
            }
            _ => {}
        }

        self.instr_pointer += next_offset;
        self.instr_pointer < self.commands.len()
    }
}

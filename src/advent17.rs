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
    println!("{a} {b} {c} {lines:?}");
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
    let mut p = Program::new(26 | (1 << (commands.len() * 3 - 1)), b, c, &commands);
    let mut num_instrs = 0;
    println!("{:?}", p.output);
    while p.next() {
        num_instrs += 1;
    }
    println!("{num_instrs}");
    println!("{:?} ", p.commands);
    println!("{:?} ", p.output);
    let mut ans = 0;
    for (index, num) in commands.iter().enumerate().rev() {
        println!("{num}");

        for i in 0..8 {
            let mut check = (i << (3 * index)) | ans;
            let mut p = Program::new(check, 0, 0, &commands);
            println!("{commands:?} {check:b}");
            let mut num_instrs = 0;
            while p.next() {
                num_instrs += 1;
            }
            println!("{:?} {check:b}", p.output);
            if p.output.len() > index && p.output[index] == *num as i64 {
                ans |= i << (index * 3);
                println!("{i} {ans:b}");
                break;
            }
        }
        let mut p = Program::new(ans, b, c, &commands);
        let mut num_instrs = 0;
        while p.next() {
            num_instrs += 1;
        }
        // println!("{:?}", p.output)
    }
    let mut p = Program::new(ans, 0, 0, &commands);
    while p.next() {
        num_instrs += 1;
    }

    // println!("{:?}", p.output);
    println!("{ans} \n {ans:b}");
}

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

use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop,
}

fn to_instruction(line: &str) -> Instruction {
    let mut split = line.split_ascii_whitespace();
    match split.next().unwrap() {
        "acc" => Instruction::Acc(split.next().unwrap().parse::<i32>().unwrap()),
        "jmp" => Instruction::Jmp(split.next().unwrap().parse::<i32>().unwrap()),
        "nop" => Instruction::Nop,
        _ => unreachable!(),
    }
}

type Program = Vec<Instruction>;

fn to_program(text: &str) -> Program {
    text.lines().map(to_instruction).collect()
}

fn find_loop(program: Program) -> i32 {
    let mut cur: i32 = 0;
    let mut acc = 0;

    let mut seen = HashSet::new();

    loop {
        if seen.contains(&cur) {
            return acc;
        }

        seen.insert(cur);

        println!("{:?}", program[cur as usize]);

        match program[cur as usize] {
            Instruction::Acc(x) => {
                acc += x;
                cur += 1;
            }
            Instruction::Jmp(x) => cur += x,
            Instruction::Nop => cur += 1,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_text = fs::read_to_string(&args[1]).unwrap();
    println!("{:?}", find_loop(to_program(&program_text)));
}

use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn to_instruction(line: &str) -> Instruction {
    let mut split = line.split_ascii_whitespace();
    match split.next().unwrap() {
        "acc" => Instruction::Acc(split.next().unwrap().parse::<i32>().unwrap()),
        "jmp" => Instruction::Jmp(split.next().unwrap().parse::<i32>().unwrap()),
        "nop" => Instruction::Nop(split.next().unwrap().parse::<i32>().unwrap()),
        _ => unreachable!(),
    }
}

type Program = Vec<Instruction>;

fn to_program(text: &str) -> Program {
    text.lines().map(to_instruction).collect()
}

fn terminates(program: &Program) -> Option<i32> {
    let mut cur: i32 = 0;
    let mut acc = 0;

    let mut seen = HashSet::new();

    loop {
        if program.len() <= cur as usize {
            return Some(acc);
        }
        if seen.contains(&cur) {
            return None;
        }

        seen.insert(cur);

        match program[cur as usize] {
            Instruction::Acc(x) => {
                acc += x;
                cur += 1;
            }
            Instruction::Jmp(x) => cur += x,
            Instruction::Nop(_) => cur += 1,
        }
    }
}

fn find_instruction(program: &mut Program) -> i32 {
    for i in 0..program.len() {
        match program[i] {
            Instruction::Acc(_) => (),
            Instruction::Jmp(x) => {
                program[i] = Instruction::Nop(x);

                match terminates(program) {
                    Some(result) => return result,
                    None => (),
                }
                program[i] = Instruction::Jmp(x);
            }
            Instruction::Nop(x) => {
                program[i] = Instruction::Jmp(x);

                match terminates(program) {
                    Some(result) => return result,
                    None => (),
                }
                program[i] = Instruction::Nop(x);
            }
        }
    }

    unreachable!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_text = fs::read_to_string(&args[1]).unwrap();
    println!("{:?}", find_instruction(&mut to_program(&program_text)));
}

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

fn seat_val(line: &str) -> u32 {
    let row = u32::from_str_radix(&line[0..7].replace("F", "0").replace("B", "1"), 2).unwrap();
    let col = u32::from_str_radix(&line[7..10].replace("L", "0").replace("R", "1"), 2).unwrap();

    return row * 8 + col;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments, expected 1 args");
        process::exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);

    let mut seats: Vec<u32> = reader
        .lines()
        .map(|line| seat_val(&line.unwrap()))
        .collect();

    seats.sort();

    for i in 0..(seats.len() - 1) {
        if seats[i] == (seats[i + 1] - 2) {
            println!("seat = {} {} ", seats[i], seats[i + 1]);
        }
    }
}

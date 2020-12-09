use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Invalid arguments, expected 2 args");
        process::exit(1);
    }

    let total = args[1].parse::<i32>().expect("arg1 not an int");

    let file = File::open(&args[2]).expect("arg2 not a file on disk");
    let reader = BufReader::new(file);

    let mut seen = HashSet::new();

    for line in reader.lines() {
        let num = line
            .unwrap()
            .parse::<i32>()
            .expect("File has a non-numeric entry");

        let val = total - num;

        if seen.contains(&val) {
            println!("Result = {}, from {} and {}", val * num, val, num);
            process::exit(0);
        }

        seen.insert(num);
    }
}

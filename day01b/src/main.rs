use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Invalid arguments, expected 2 args");
        process::exit(1);
    }

    let total = args[1].parse::<i32>().expect("arg1 not an int");

    let file = File::open(&args[2]).expect("arg2 not a file on disk");
    let reader = BufReader::new(file);

    let mut data = HashSet::new();

    for line in reader.lines() {
        let num = line
            .unwrap()
            .parse::<i32>()
            .expect("File has a non-numeric entry");

        let to_find = total - num;

        if to_find <= 0 {
            continue;
        }

        for rest in &data {
            let val = to_find - rest;

            if val <= 0 {
                continue;
            }

            if data.contains(&val) {
                println!(
                    "Result = {}, from {}, {} and {}",
                    num * rest * val,
                    num,
                    rest,
                    val
                );
                process::exit(0);
            }
        }

        data.insert(num);
    }

    println!("no match found at all");
    process::exit(1);
}

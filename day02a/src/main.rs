use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

#[macro_use]
extern crate lazy_static;

fn is_valid(password: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<min>\d+)-(?P<max>\d+)\s*(?P<char>\w): (?P<password>\w+)$").unwrap();
    }

    match RE.captures(password) {
        None => {
            println!("Password {} did not match regex", password);
            return false;
        }
        Some(captures) => {
            let min = captures
                .name("min")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let max = captures
                .name("max")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            let c = captures.name("char").unwrap().as_str();

            let pwd = captures.name("password").unwrap().as_str();

            let count = pwd.matches(c).count();

            return min <= count && count <= max;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments, expected 1 args");
        process::exit(1);
    }

    let file = File::open(&args[1]).expect("arg1 not a file on disk");
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        if is_valid(&line.unwrap()) {
            count += 1;
        }
    }

    println!("Count = {}", count);
}

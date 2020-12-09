use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments, expected 1 args");
        process::exit(1);
    }

    let file = File::open(&args[1]).expect("arg1 not a file on disk");
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut position = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();
        let cur_line = line_str.as_bytes();

        if cur_line[position % cur_line.len()] as char == '#' {
            count += 1;
        }

        position += 3;
    }

    println!("Tree count = {}", count);
}

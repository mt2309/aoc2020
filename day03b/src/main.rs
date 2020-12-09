use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

struct State {
    right: usize,
    down: usize,

    count: usize,
    pos: usize,
}

fn is_tree(line: &[u8], pos: usize) -> bool {
    return line[pos % line.len()] as char == '#';
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments, expected 1 args");
        process::exit(1);
    }

    let mut routes: [State; 5] = [
        State {
            right: 1,
            down: 1,
            count: 0,
            pos: 0,
        },
        State {
            right: 3,
            down: 1,
            count: 0,
            pos: 0,
        },
        State {
            right: 5,
            down: 1,
            count: 0,
            pos: 0,
        },
        State {
            right: 7,
            down: 1,
            count: 0,
            pos: 0,
        },
        State {
            right: 1,
            down: 2,
            count: 0,
            pos: 0,
        },
    ];

    let file = File::open(&args[1]).expect("arg1 not a file on disk");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();
        let cur_line = line_str.as_bytes();

        for route in routes.iter_mut() {
            if index % route.down != 0 {
                continue;
            }

            if is_tree(cur_line, route.pos) {
                route.count += 1;
            }

            route.pos += route.right;
        }
    }

    println!(
        "Tree count = {}",
        routes.iter().fold(1, |val, route| val * route.count)
    );
}

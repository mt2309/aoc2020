use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn find_invalid(cipher: &str, preamble: usize) -> i32 {
    let mut lines = cipher.lines().map(str::parse::<i32>).map(Result::unwrap);
    let mut preamble: VecDeque<i32> = lines.by_ref().take(preamble).collect();

    let mut current = HashSet::new();
    for i in preamble.iter() {
        current.insert(*i);
    }

    for val in lines {
        let mut found = false;
        for i in preamble.iter().by_ref() {
            match current.get(&(val - i)) {
                None => continue,
                Some(_) => {
                    found = true;
                    break;
                }
            }
        }
        println!("looking at {}, preamble = {:?}", val, preamble);
        if !found {
            return val;
        }
        let head = preamble.pop_front().unwrap();
        current.remove(&head);
        preamble.push_back(val);
        current.insert(val);
    }

    unreachable!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cipher = fs::read_to_string(&args[1]).unwrap();
    println!("{}", find_invalid(&cipher, 25));
}

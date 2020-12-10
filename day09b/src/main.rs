use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn find_invalid(cipher: &Vec<i64>, preamble_len: usize) -> i64 {
    let mut preamble: VecDeque<i64> = cipher.iter().take(preamble_len).map(|x| *x).collect();

    let mut current: HashSet<i64> = HashSet::new();
    for i in preamble.iter() {
        current.insert(*i);
    }

    for val in cipher.iter().skip(preamble_len) {
        let mut found = false;
        for i in preamble.iter().by_ref() {
            match current.get(&(val - *i)) {
                None => continue,
                Some(_) => {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return *val;
        }
        let head: i64 = preamble.pop_front().unwrap();
        current.remove(&head);
        preamble.push_back(*val);
        current.insert(*val);
    }

    unreachable!();
}

fn find_sequence(cipher: Vec<i64>, value: i64) -> i64 {
    for i in 0..cipher.len() {
        for j in i..cipher.len() {
            let slice: &[i64] = &cipher[i..j];
            let total: i64 = slice.iter().sum();

            if total == value {
                let (min, max) = slice.iter().fold((i64::MAX, i64::MIN), |(min, max), x| {
                    (cmp::min(min, *x), cmp::max(max, *x))
                });

                return min + max;
            } else if total > value {
                break;
            }
        }
    }

    unreachable!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cipher: Vec<i64> = fs::read_to_string(&args[1])
        .unwrap()
        .lines()
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();

    let weakness = find_invalid(&cipher, 25);
    let sequence = find_sequence(cipher, weakness);
    println!("weakness = {}", sequence);
}

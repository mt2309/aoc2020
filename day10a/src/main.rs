use itertools::sorted;
use std::env;
use std::fs;

fn find_jolts_alt(adapters: Vec<i32>) -> i32 {
    let (one, three, _) = adapters
        .iter()
        .fold((0, 1, 0), |(one, three, current), adapter| {
            match adapter - current {
                1 => (one + 1, three, *adapter),
                3 => (one, three + 1, *adapter),
                _ => (one, three, *adapter),
            }
        });

    one * three
}

fn main() {
    // jolts = 2210
    let args: Vec<String> = env::args().collect();
    let adapters = fs::read_to_string(&args[1]).unwrap();
    println!(
        "jolts = {}",
        find_jolts_alt(
            sorted(adapters.lines().map(str::parse::<i32>).map(Result::unwrap)).collect()
        )
    );
}

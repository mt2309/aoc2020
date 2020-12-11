use itertools::sorted;
use std::collections::HashMap;
use std::env;
use std::fs;

fn find_jolts_alt(adapters: &mut Vec<i32>) -> i64 {
    adapters.insert(0, 0);

    let mut cache: HashMap<i32, i64> = HashMap::new();
    cache.insert(3 + *adapters.last().unwrap(), 1);

    adapters.iter().rev().for_each(|next| {
        let count = ((next + 1)..(next + 4))
            .map(|x| cache.get(&x).unwrap_or(&0))
            .sum();
        cache.insert(*next, count);
    });

    *cache.get(&0).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let adapters = fs::read_to_string(&args[1]).unwrap();

    println!(
        "jolts = {}",
        find_jolts_alt(
            &mut sorted(adapters.lines().map(str::parse::<i32>).map(Result::unwrap)).collect()
        )
    );
}

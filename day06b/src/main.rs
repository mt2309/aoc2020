#![feature(iterator_fold_self)]
use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter::FromIterator;
use std::process;

fn question_total(section: &str) -> usize {
    section
        .lines()
        .map(|person| HashSet::from_iter(person.chars()))
        .fold_first(|a, b| a.intersection(&b).cloned().collect::<HashSet<char>>())
        .map(|x| x.len())
        .unwrap_or(0)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments, expected 1 arg");
        process::exit(1);
    }

    let questions_str = fs::read_to_string(&args[1]).unwrap();

    let sum: usize = questions_str
        .split("\n\n")
        .map(|question| question_total(question))
        .sum();
    println!("sum = {}", sum);
}

use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref REQ_KEYS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("byr");
        s.insert("iyr");
        s.insert("eyr");
        s.insert("hgt");
        s.insert("hcl");
        s.insert("ecl");
        s.insert("pid");
        s
    };
}

lazy_static! {
    static ref OPT_KEYS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("cid");
        s
    };
}

fn is_valid(passport: &str) -> bool {
    let mut seen = HashSet::new();
    for kv in passport.split_ascii_whitespace() {
        let key = kv.splitn(2, ":").next().unwrap();

        if REQ_KEYS.contains(key) {
            seen.insert(key);
        } else if !OPT_KEYS.contains(key) {
            return false;
        }
    }

    return seen.len() == REQ_KEYS.len();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments, expected 1 arg");
        process::exit(1);
    }

    let passports_str = fs::read_to_string(&args[1]).unwrap();
    let passports: Vec<&str> = passports_str.split("\n\n").collect();

    println!(
        "valid passports = {}",
        passports.iter().fold(0, |count, passport| count
            + if is_valid(passport) { 1 } else { 0 })
    );
}

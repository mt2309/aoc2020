use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

#[macro_use]
extern crate lazy_static;

type Validate = fn(&str) -> bool;

fn valid_year(year: &str, min: u32, max: u32) -> bool {
    if year.len() != 4 {
        return false;
    }

    match year.parse::<u32>() {
        Ok(yr) => yr >= min && yr <= max,
        _ => false,
    }
}

fn valid_birth_year(year: &str) -> bool {
    return valid_year(year, 1920, 2002);
}

fn valid_issue_year(year: &str) -> bool {
    return valid_year(year, 2010, 2020);
}

fn valid_expire_year(year: &str) -> bool {
    return valid_year(year, 2020, 2030);
}

fn valid_height(height: &str) -> bool {
    match height.strip_suffix("cm") {
        Some(cm) => match cm.parse::<u32>() {
            Ok(v) => v >= 150 && v <= 193,
            _ => false,
        },
        None => match height.strip_suffix("in") {
            Some(inch) => match inch.parse::<u32>() {
                Ok(v) => v >= 59 && v <= 76,
                _ => false,
            },
            None => false,
        },
    }
}

fn valid_hair_colour(colour: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }

    return RE.is_match(colour);
}

fn valid_eye_colour(colour: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    }

    return RE.is_match(colour);
}

fn valid_id(id: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    return RE.is_match(id);
}

lazy_static! {
    static ref REQ_KEYS: HashMap<&'static str, Validate> = {
        let mut m = HashMap::new();
        m.insert("byr", valid_birth_year as fn(&str) -> bool);
        m.insert("iyr", valid_issue_year);
        m.insert("eyr", valid_expire_year);
        m.insert("hgt", valid_height);
        m.insert("hcl", valid_hair_colour);
        m.insert("ecl", valid_eye_colour);
        m.insert("pid", valid_id);
        m
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
        let mut split = kv.splitn(2, ":");
        let key = split.next().unwrap();
        let val = split.next().unwrap();

        if OPT_KEYS.contains(key) {
            continue;
        }

        let validator = REQ_KEYS.get(&key);

        if validator.is_some() && validator.unwrap()(val) {
            seen.insert(key);
        } else {
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

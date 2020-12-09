use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn iter_to_colour<'a>(iter: &mut impl DoubleEndedIterator<Item = &'a str>) -> String {
    let mut colour = iter.next().unwrap().to_string();
    colour.push_str(" ");
    colour.push_str(iter.next().unwrap());

    return colour;
}

fn iter_to_count<'a>(
    iter: &mut impl DoubleEndedIterator<Item = &'a str>,
) -> Option<(usize, String)> {
    let count = match iter.next().unwrap() {
        "no" => return None,
        val => val.parse::<usize>().unwrap(),
    };

    Some((count, iter_to_colour(iter)))
}

type Rules = HashMap<String, Vec<(usize, String)>>;

fn make_rules(rules_str: &str) -> Rules {
    let mut rules: Rules = HashMap::new();

    for rule in rules_str.lines() {
        let mut tok_iter = rule.split(|c: char| !c.is_ascii_alphanumeric() && c != '.' && c != ',');
        let colour = iter_to_colour(&mut tok_iter);

        // skip "bags contain"
        tok_iter.next();
        tok_iter.next();

        let mut contains = Vec::new();
        loop {
            match iter_to_count(&mut tok_iter) {
                Some(item) => {
                    contains.push(item);

                    // pop next item, should be "bags?[,.]"
                    let bag = tok_iter.next().unwrap();
                    if bag.ends_with(".") {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }

        rules.insert(colour, contains);
    }
    rules
}

fn paths_to_gold(rules: Rules) -> usize {
    let mut containers: HashMap<String, HashSet<String>> = HashMap::new();

    for (container, bags) in rules {
        for (_, bag) in bags {
            containers
                .entry(bag)
                .or_insert_with(HashSet::new)
                .insert(container.clone());
        }
    }

    let mut result: HashSet<String> = HashSet::new();
    result.extend(
        containers
            .get("shiny gold")
            .unwrap()
            .iter()
            .map(String::to_string),
    );

    let mut queue: VecDeque<String> = result.iter().map(String::to_string).collect();

    while !queue.is_empty() {
        let elem: String = queue.pop_front().unwrap();
        match containers.get(&elem) {
            Some(p) => {
                let parent: Vec<String> = p.iter().map(String::to_string).collect();
                result.extend(parent.clone());
                queue.extend(parent.clone());
            }
            _ => (),
        }
    }

    return result.len();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!(
        "{:?}",
        paths_to_gold(make_rules(&fs::read_to_string(&args[1]).unwrap()))
    );
}

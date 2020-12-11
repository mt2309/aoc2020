use std::cmp;
use std::env;
use std::fmt;
use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match &self {
            Position::Floor => ".",
            Position::Empty => "L",
            Position::Occupied => "#",
        })
    }
}

fn c2p(c: char) -> Position {
    match c {
        '.' => Position::Floor,
        'L' => Position::Empty,
        '#' => Position::Occupied,
        _ => unreachable!(),
    }
}

type Map = Vec<Vec<Position>>;

fn make_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(c2p).collect())
        .collect()
}

fn count_occupied(i: usize, j: usize, input: &Map) -> usize {
    (i.saturating_sub(1)..cmp::min(i + 2, input.len()))
        .map(|x: usize| {
            (j.saturating_sub(1)..cmp::min(j + 2, input[x].len()))
                .map(|y: usize| match input[x][y] {
                    Position::Occupied => 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

fn it(i: usize, j: usize, pos: Position, input: &Map) -> Position {
    match pos {
        Position::Floor => Position::Floor,
        Position::Empty => {
            if count_occupied(i, j, input) == 0 {
                Position::Occupied
            } else {
                Position::Empty
            }
        }
        Position::Occupied => {
            // >4 to include current position
            if count_occupied(i, j, input) > 4 {
                Position::Empty
            } else {
                Position::Occupied
            }
        }
    }
}

fn iterate(input: &Map) -> Map {
    input
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, pos)| it(i, j, *pos, &input))
                .collect()
        })
        .collect()
}

fn stable(input: &mut Map) -> usize {
    loop {
        let next = iterate(&input);
        if next == *input {
            return next
                .iter()
                .map(|row| {
                    row.into_iter()
                        .filter(|x| **x == Position::Occupied)
                        .count()
                })
                .sum();
        } else {
            *input = next;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    println!("{:?}", stable(&mut make_map(&input)));
}

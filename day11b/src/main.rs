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

fn line_occupied(i: usize, j: usize, x: i32, y: i32, input: &Map) -> usize {
    if x == 0 && y == 0 {
        return 0;
    } else {
        for n in 1.. {
            let x_ = i as i32 + (x * n);
            let y_ = j as i32 + (y * n);

            if x_ < 0 || x_ >= input.len() as i32 {
                return 0;
            }
            if y_ < 0 || y_ >= input[x_ as usize].len() as i32 {
                return 0;
            }

            match input[x_ as usize][y_ as usize] {
                Position::Occupied => return 1,
                Position::Empty => return 0,
                Position::Floor => (),
            }
        }
    }

    unreachable!();
}

fn count_occupied(i: usize, j: usize, input: &Map) -> usize {
    (-1..2)
        .map(|x| {
            (-1..2)
                .map(|y| line_occupied(i, j, x, y, input))
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
            if count_occupied(i, j, input) >= 5 {
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
                .map(|(j, pos)| it(i, j, *pos, input))
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

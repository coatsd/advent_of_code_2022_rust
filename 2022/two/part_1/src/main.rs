use std::cmp::{Eq, PartialEq};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let buf = BufReader::new(open_file("./input.txt"));
    let result = buf.lines().into_iter().fold(0, |mut acc, l| {
        if let Ok(l) = l {
            acc += calc_line(&l);
        }
        acc
    });
    println!("{}", result);
}

#[derive(Eq, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn new(c: char) -> Option<Self> {
        use Hand::*;
        match c {
            'A' | 'X' => Some(Rock),
            'B' | 'Y' => Some(Paper),
            'C' | 'Z' => Some(Scissors),
            _ => None,
        }
    }
    fn get_value(&self) -> i32 {
        use Hand::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

enum HandResult {
    Win,
    Draw,
    Lose,
}

impl HandResult {
    fn get_result(hand: Hand, other: Hand) -> Self {
        use Hand::*;
        use HandResult::*;
        if hand == other {
            return Draw;
        }
        match (hand, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Win,
            _ => Lose,
        }
    }
    fn get_value(&self) -> i32 {
        use HandResult::*;
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

fn calc_line(l: &String) -> i32 {
    let (mut hand, mut other) = (Option::<Hand>::None, Option::<Hand>::None);
    for c in l.chars() {
        if c == ' ' {
            continue;
        }
        match c {
            'X' | 'Y' | 'Z' => {
                hand = Hand::new(c);
            }
            'A' | 'B' | 'C' => {
                other = Hand::new(c);
            }
            invalid => panic!("Found invalid char in input {}", invalid),
        };
    }
    if let (Some(hand), Some(other)) = (hand, other) {
        return hand.get_value() + HandResult::get_result(hand, other).get_value();
    }
    0
}

fn open_file<P>(path: P) -> File
where
    P: AsRef<Path> + std::fmt::Display,
{
    let file = std::fs::File::open(&path);
    match file {
        Ok(f) => f,
        Err(e) => panic!("Could not open file {}: {}", path, e),
    }
}

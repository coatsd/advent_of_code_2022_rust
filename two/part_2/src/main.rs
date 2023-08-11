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
            'A' => Some(Rock),
            'B' => Some(Paper),
            'C' => Some(Scissors),
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
    fn get_required(&self, action: &HandResult) -> Hand {
        use Hand::*;
        use HandResult::*;
        if *action == Draw {
            return match self {
                Rock => Rock,
                Paper => Paper,
                Scissors => Scissors,
            };
        }
        match (self, action) {
            (Rock, Win) | (Scissors, Lose) => Paper,
            (Paper, Win) | (Rock, Lose) => Scissors,
            _ => Rock,
        }
    }
}

#[derive(Eq, PartialEq)]
enum HandResult {
    Win,
    Draw,
    Lose,
}

impl HandResult {
    fn new(c: char) -> Option<Self> {
        use HandResult::*;
        match c {
            'X' => Some(Lose),
            'Y' => Some(Draw),
            'Z' => Some(Win),
            _ => None,
        }
    }
    fn get_result(hand: &Hand, other: &Hand) -> Self {
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
    let mut opp_hand = Option::<Hand>::None;
    let mut action = Option::<HandResult>::None;
    for c in l.chars() {
        if c == ' ' {
            continue;
        }
        match c {
            'A' | 'B' | 'C' => {
                opp_hand = Hand::new(c);
            }
            'X' | 'Y' | 'Z' => {
                action = HandResult::new(c);
            }
            invalid => panic!("Found invalid char in input {}", invalid),
        };
    }
    if let (Some(opp_hand), Some(action)) = (opp_hand, action) {
        let required_hand = Hand::get_required(&opp_hand, &action);
        return required_hand.get_value()
            + HandResult::get_result(&required_hand, &opp_hand).get_value();
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

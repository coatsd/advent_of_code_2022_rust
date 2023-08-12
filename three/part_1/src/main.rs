use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let buf = BufReader::new(open_file("./input.txt"));
    let mut total = 0;
    for l in buf.lines() {
        if let Ok(l) = l {
            let line_priority = handle_rucksack(&l);
            total += line_priority;
            if line_priority == 0 {
                println!("Found invalid line {}", l);
            }
        }
    }
    println!("{}", total);
}

fn get_priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 96,
        'A'..='Z' => c as i32 - 38,
        invalid => panic!("Invalid character detected in input: {}", invalid),
    }
}

fn handle_rucksack(l: &String) -> i32 {
    let mid = l.len() / 2;
    let mut items_left = String::new();
    let mut items_right = String::new();
    let handle_item = |items: &mut String, c: char| {
        if !items.contains(c) {
            items.push(c);
        }
    };
    for (i, c) in l.chars().enumerate() {
        if i < mid {
            handle_item(&mut items_left, c);
            continue;
        }
        handle_item(&mut items_right, c);
    }
    for i in items_left.chars() {
        if items_right.contains(i) {
            return get_priority(i);
        }
    }
    0
}

fn open_file<P>(path: P) -> File
where
    P: AsRef<Path> + std::fmt::Display,
{
    let file = File::open(&path);
    match file {
        Ok(f) => f,
        Err(e) => panic!("Could not open file {}: {}", path, e),
    }
}

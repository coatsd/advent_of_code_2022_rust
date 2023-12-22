use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let buf = BufReader::new(open_file("./input.txt"));
    let mut result: i32 = 0;
    for l in buf.lines() {
        if let Ok(l) = l {
            result += match l.len() {
                0 => 0,
                _ => parse_line_to_i32(l),
            };
        }
    }
    println!("{}", result);
}

fn parse_line_to_i32(l: String) -> i32 {
    let mut digits: [Option<i32>; 2] = [None, None];
    for c in l.chars() {
        if let Some(c) = parse_digit(c) {
            digits[0] = Some(c);
            break;
        }
    }
    for c in l.chars().rev() {
        if let Some(c) = parse_digit(c) {
            digits[1] = Some(c);
            break;
        }
    }
    let mut result: i32 = 0;
    for (i, d) in digits.iter().enumerate() {
        let power = digits.len() - (i + 1);
        result += match d {
            Some(dig) => dig * (10 as i32).pow(power as u32),
            None => {
                println!("Line {l} failed to produce two digits!");
                0
            }
        };
    }
    return result;
}

fn parse_digit(c: char) -> Option<i32> {
    return if c.is_ascii_digit() {
        Some((c as u32 - 48) as i32)
    } else {
        None
    };
}

fn open_file<P>(path: P) -> File
where
    P: AsRef<Path> + std::fmt::Display,
{
    let file = std::fs::File::open(&path);
    match file {
        Ok(file) => file,
        Err(e) => panic!("Could not open file {}: {}", path, e),
    }
}

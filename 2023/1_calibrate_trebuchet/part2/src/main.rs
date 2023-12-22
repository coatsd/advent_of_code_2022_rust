use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const SPELLEDNUMERICS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

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
    for i in 1..=l.len() {
        if let Some(c) = convert_slice_to_i32(&l[0..i]) {
            digits[0] = Some(c);
            break;
        }
    }
    for i in (0..l.len()).rev() {
        if let Some(c) = convert_slice_to_i32(&l[i..l.len()]) {
            digits[1] = Some(c);
            break;
        }
    }
    let mut result: i32 = 0;
    for (i, d) in digits.iter().enumerate() {
        match d {
            Some(dig) => {
                let power = digits.len() - (i + 1);
                result += dig * (10 as i32).pow(power as u32);
            }
            None => {
                let d1 = match digits[0] {
                    Some(dig) => dig.to_string(),
                    None => String::from("None"),
                };
                let d2 = match digits[1] {
                    Some(dig) => dig.to_string(),
                    None => String::from("None"),
                };
                println!(
                    "Line {} failed to produce two digits! Digits produced: 1st - {}, 2nd - {}",
                    l, d1, d2
                );
                break;
            }
        };
    }
    return result;
}

fn convert_slice_to_i32(s: &str) -> Option<i32> {
    for c in '0'..='9' {
        if s.contains(c) {
            return Some((c as u32 - 48) as i32);
        }
    }
    for spelled_num in SPELLEDNUMERICS {
        if s.to_lowercase().contains(spelled_num) {
            return Some(parse_spelled_numeric(spelled_num));
        }
    }
    return None;
}

fn parse_spelled_numeric(s: &str) -> i32 {
    return match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    };
}

fn open_file<P>(path: P) -> File
where
    P: AsRef<Path> + std::fmt::Display,
{
    let file = File::open(&path);
    match file {
        Ok(file) => file,
        Err(e) => panic!("Could not open file {}: {}", path, e),
    }
}

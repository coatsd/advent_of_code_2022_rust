use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let buf = BufReader::new(open_file("./input.txt"));
    let mut highest = [0, 0, 0];
    let mut current_total = 0;
    for l in buf.lines() {
        if let Ok(l) = l {
            match l.parse::<i32>() {
                Ok(n) => {
                    current_total += n;
                }
                Err(_) => {
                    handle_blank_line(&mut highest, current_total);
                    current_total = 0;
                }
            }
        }
    }
    let mut acc = 0;
    for h in highest {
        acc += h;
    }
    println!("{}", acc);
}

fn handle_blank_line(highest: &mut [i32; 3], current_total: i32) {
    let mut temp_h = 0;
    let mut is_higher = false;
    for i in 0..highest.len() {
        if is_higher {
            let last_value = highest[i];
            highest[i] = temp_h;
            temp_h = last_value;
            continue;
        }
        if highest[i] < current_total {
            is_higher = true;
            temp_h = highest[i];
            highest[i] = current_total;
        }
    }
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

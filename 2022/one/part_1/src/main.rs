use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let buf = BufReader::new(open_file("./input.txt"));
    let mut highest = 0;
    let mut current_total = 0;
    for l in buf.lines() {
        if let Ok(l) = l {
            match l.parse::<i32>() {
                Ok(n) => {
                    current_total += n;
                }
                Err(_) => {
                    if highest < current_total {
                        highest = current_total;
                    }
                    current_total = 0;
                }
            }
        }
    }
    println!("{}", highest);
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

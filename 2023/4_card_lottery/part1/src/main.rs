use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const TESTFILEPATH: &str = "./test_input.txt";
const FILEPATH: &str = "./input.txt";

fn main() {
    let is_test: bool = env::args()
        .collect::<Vec<String>>()
        .contains(&"-t".to_string());
    let file_path = match is_test {
        true => TESTFILEPATH,
        _ => FILEPATH,
    };
    let buf = BufReader::new(open_file(file_path));

    let mut result: i32 = 0;
    for (i, l) in buf.lines().enumerate() {
        match l {
            Ok(l) => {
                if l.len() == 0 {
                    continue;
                }
                let card = match Card::parse(l) {
                    Ok(c) => c,
                    Err(e) => panic!("{}", e),
                };
                let score = card.calc_score();
                if is_test {
                    println!(
                        "Card {}: winning nums: {:?} | our nums {:?} = score: {}",
                        i + 1,
                        card.winning_nums,
                        card.our_nums,
                        score
                    );
                }
                result += score;
            }
            Err(e) => panic!("{}", e.to_string()),
        }
    }
    println!("{}", result);
}

struct Card {
    winning_nums: Vec<u8>,
    our_nums: Vec<u8>,
}
impl Card {
    pub fn parse(line: String) -> Result<Self, String> {
        let card_trim_index = match line.find(':') {
            Some(i) => i,
            None => return Err(format!("Line contains no colon:\n\t{}", line)),
        };
        let pipe_index = match line.find('|') {
            Some(i) => i,
            None => return Err(format!("Line contains no pipe:\n\t{}", line)),
        };
        let (winning_nums, our_nums) = (
            Self::parse_nums(&line[card_trim_index + 1..pipe_index])?,
            Self::parse_nums(&line[pipe_index + 1..line.len()])?,
        );

        return Ok(Self {
            winning_nums,
            our_nums,
        });
    }

    pub fn calc_score(&self) -> i32 {
        if self.our_nums.len() == 0 || self.winning_nums.len() == 0 {
            return 0;
        }

        let mut winning_count: i32 = -1;
        for winning_num in self.winning_nums.iter() {
            if self.our_nums.contains(&winning_num) {
                winning_count += 1;
            }
        }

        return if winning_count == -1 {
            0
        } else {
            (2 as i32).pow(winning_count as u32)
        };
    }

    fn parse_nums(s: &str) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        let mut num_string: String = "".to_string();

        for (i, c) in s.chars().enumerate() {
            match c {
                '0'..='9' => {
                    num_string.push(c);
                    if i + 1 == s.len() {
                        match num_string.parse::<u8>() {
                            Ok(n) => result.push(n),
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                }
                _ => {
                    if num_string.len() == 0 {
                        continue;
                    }
                    match num_string.parse::<u8>() {
                        Ok(n) => result.push(n),
                        Err(e) => return Err(e.to_string()),
                    }
                    num_string.clear();
                }
            }
        }

        return Ok(result);
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

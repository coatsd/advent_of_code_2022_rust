use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let buf = BufReader::new(open_file("./input.txt"));
    let schematic = match Schematic::parse_schematic(buf) {
        Ok(s) => s,
        Err(e) => panic!("Failed to parse schematic!\n Error: {}", e),
    };
    let part_nums = match schematic.find_part_nums() {
        Ok(part_nums) => part_nums,
        Err(e) => panic!("Failed to parse part_nums!\n Error: {}", e),
    };

    let mut result = 0;
    for part_num in part_nums {
        result += part_num;
    }

    println!("{}", result);
}

enum Symbol {
    Digit(char),
    Period,
    Special,
}
impl Symbol {
    pub fn parse_char(c: char) -> Self {
        return match c {
            '0'..='9' => Symbol::Digit(c),
            '.' => Symbol::Period,
            _ => Symbol::Special,
        };
    }
}

struct Schematic(Vec<Vec<Symbol>>);
impl Schematic {
    pub fn parse_schematic(buf: BufReader<File>) -> Result<Schematic, std::io::Error> {
        let mut result: Vec<Vec<Symbol>> = Vec::new();
        for l in buf.lines() {
            let l = l?;
            if l.len() == 0 {
                continue;
            }
            let mut sym_vec = Vec::new();
            for c in l.chars() {
                sym_vec.push(Symbol::parse_char(c));
            }
            result.push(sym_vec);
        }
        return Ok(Schematic(result));
    }

    pub fn find_part_nums(&self) -> Result<Vec<i32>, String> {
        if self.0.len() == 0 {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();

        let append_to_result = |numeric_string: &mut String,
                                is_part_num: &mut bool,
                                result: &mut Vec<i32>|
         -> Result<(), String> {
            if numeric_string.len() > 0 && *is_part_num {
                let part_num = match numeric_string.parse::<i32>() {
                    Ok(n) => n,
                    Err(e) => return Err(e.to_string()),
                };
                result.push(part_num);
            }
            numeric_string.clear();
            *is_part_num = false;
            return Ok(());
        };

        for (i, sym_vec) in self.0.iter().enumerate() {
            let mut numeric_string = "".to_string();
            let mut is_part_num = false;
            for (j, sym) in sym_vec.iter().enumerate() {
                match sym {
                    Symbol::Digit(d) => {
                        numeric_string.push(*d);
                        if !is_part_num {
                            is_part_num = self.has_adjacent_special_char(i, j)?;
                        }
                        if j == sym_vec.len() - 1 {
                            if let Err(e) =
                                append_to_result(&mut numeric_string, &mut is_part_num, &mut result)
                            {
                                panic!("{}", e);
                            }
                        }
                    }
                    _ => {
                        if let Err(e) =
                            append_to_result(&mut numeric_string, &mut is_part_num, &mut result)
                        {
                            panic!("{}", e)
                        }
                    }
                }
            }
        }

        return Ok(result);
    }

    fn has_adjacent_special_char(&self, i: usize, j: usize) -> Result<bool, String> {
        if self.0.len() == 0 {
            return Err(
                "Schematic contains no lines, and therefore cannot contain special characters!"
                    .to_string(),
            );
        }
        if i >= self.0.len() {
            return Err(format!(
                "argument i is out of range. argument: {}, length of schematic.0: {}",
                i,
                self.0.len(),
            ));
        }
        if j >= self.0[i].len() {
            return Err(format!(
                "argument j is out of range. argument: {}, length of schematic.0[{}]: {}",
                i,
                j,
                self.0[0].len()
            ));
        }

        let i = i as i32;
        let j = j as i32;

        for a in -1..=1 {
            let x = i + a;
            for b in -1..=1 {
                if a == 0 && b == 0 {
                    continue;
                }

                let y = j + b;

                if x < 0 || x >= (self.0.len() as i32) {
                    continue;
                }
                if y < 0 || y >= (self.0[0].len() as i32) {
                    continue;
                }

                if let Symbol::Special = self.0[x as usize][y as usize] {
                    return Ok(true);
                }
            }
        }

        return Ok(false);
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

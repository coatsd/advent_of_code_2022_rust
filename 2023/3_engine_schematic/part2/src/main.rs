use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::usize;

fn main() {
    let buf = BufReader::new(open_file("./input.txt"));
    let schematic = match Schematic::parse_schematic(buf) {
        Ok(s) => s,
        Err(e) => panic!("Failed to parse schematic!\n Error: {}", e),
    };
    let gears = match schematic.find_gear_info() {
        Ok(part_nums) => part_nums,
        Err(e) => panic!("Failed to parse part_nums!\n Error: {}", e),
    };

    let mut result = 0;
    for gear in gears {
        if gear.adjacent_part_nums.len() < 2 {
            continue;
        }
        println!(
            "Gear at {:?} - Adjacent parts: {:?}",
            gear.gear_location, gear.adjacent_part_nums
        );
        result += gear.get_gear_ratio();
    }

    println!("{}", result);
}

enum Symbol {
    Digit(char),
    Period,
    Gear,
    Special,
}
impl Symbol {
    pub fn parse_char(c: char) -> Self {
        return match c {
            '0'..='9' => Symbol::Digit(c),
            '.' => Symbol::Period,
            '*' => Symbol::Gear,
            _ => Symbol::Special,
        };
    }
}

struct GearInfo {
    gear_location: [usize; 2],
    adjacent_part_nums: Vec<i32>,
}
impl GearInfo {
    pub fn new(gear_location: [usize; 2], adjacent_part_nums: Vec<i32>) -> Self {
        return Self {
            gear_location,
            adjacent_part_nums,
        };
    }

    pub fn get_gear_ratio(&self) -> i32 {
        let mut result = 0;
        for part_num in self.adjacent_part_nums.iter() {
            result = match result {
                0 => *part_num,
                _ => *part_num * result,
            }
        }
        return result;
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

    pub fn find_gear_info(&self) -> Result<Vec<GearInfo>, String> {
        if self.0.len() == 0 {
            return Ok(Vec::new());
        }

        let mut result: Vec<GearInfo> = Vec::new();

        for (i, sym_vec) in self.0.iter().enumerate() {
            for (j, sym) in sym_vec.iter().enumerate() {
                match sym {
                    Symbol::Gear => {
                        let adjacent_part_nums = self.parse_adjacent_part_nums(i, j)?;
                        result.push(GearInfo::new([i, j], adjacent_part_nums));
                    }
                    _ => (),
                }
            }
        }

        return Ok(result);
    }

    fn parse_adjacent_part_nums(&self, i: usize, j: usize) -> Result<Vec<i32>, String> {
        // check if the indices passed in are in-bounds of the Scematic.
        if self.0.len() == 0 {
            return Err(
                "Schematic contains no lines, and therefore cannot contain adjacent part numbers!"
                    .to_string(),
            );
        }
        if i >= self.0.len() {
            return Err(format!(
                "argument i is out of range. argument: {}, length of schematic.0: {}",
                i,
                self.0.len()
            ));
        }
        if j >= self.0[0].len() {
            return Err(format!(
                "argument j is out of range. argument: {}, length of schematic.0[{}]: {}",
                j,
                i,
                self.0[i].len()
            ));
        }

        // parse the usizes passed in as i32, so they can be negative for bounds checking.
        let i = i as i32;
        let j = j as i32;

        // create a vector of visited nodes, to be sure we don't try to parse the same adjacent
        // numbers.
        let mut visited: Vec<[usize; 2]> = Vec::new();
        let was_visited = |coord: [usize; 2], visited: &Vec<[usize; 2]>| -> bool {
            for &v in visited.iter() {
                if v[0] == coord[0] && v[1] == coord[1] {
                    return true;
                }
            }
            return false;
        };

        // create result that will be appended to.
        let mut result = Vec::new();

        // loop through the adjacent indices around [i, j] and check if they're Digits.
        for a in -1..=1 {
            let x = i + a;
            for b in -1..=1 {
                // No need to check if it's the gear node.
                if a == 0 && b == 0 {
                    continue;
                }

                let y = j + b;

                // Make sure we're in bounds of the Schematic.
                if x < 0 || x >= (self.0.len() as i32) {
                    continue;
                }
                if y < 0 || y >= (self.0[0].len() as i32) {
                    continue;
                }

                // We know we're in bounds at this point, so parse as a usize to index
                // into array
                let x = x as usize;
                let y = y as usize;

                // Check to be sure we haven't visited this node.
                if was_visited([x, y], &visited) {
                    continue;
                }

                // If the node we're looking at is a Digit, start parsing from left to right,
                // making sure we append to our visited nodes along the way.
                if let Symbol::Digit(d) = self.0[x][y] {
                    visited.push([x, y]);

                    // keep track of the numerics that we find while crawling to the
                    // left and right.
                    let mut numeric_string: String = String::new();
                    numeric_string.push(d);

                    // make left_index signed, so it cango negative and break the loop.
                    let mut left_index = y as i32 - 1;
                    let mut right_index = y + 1;

                    // have the left index crawl to the left, appending Digits it finds
                    // to the beginning of numeric_string. Break if not a Digit.
                    while left_index >= 0 {
                        if was_visited([x, left_index as usize], &visited) {
                            break;
                        }
                        if let Symbol::Digit(d) = self.0[x][left_index as usize] {
                            visited.push([x, left_index as usize]);
                            numeric_string.insert(0, d);
                            left_index -= 1;
                        } else {
                            break;
                        }
                    }
                    // have the right_index crawl to the right, appending Digits it finds
                    // to the end of numeric_string. Break if not a Digit.
                    while right_index < self.0[x].len() {
                        if was_visited([x, right_index], &visited) {
                            break;
                        }
                        if let Symbol::Digit(d) = self.0[x][right_index] {
                            visited.push([x, right_index]);
                            numeric_string.push(d);
                            right_index += 1;
                        } else {
                            break;
                        }
                    }

                    // parse the string and append it to the result of the method.
                    // If there's an error parsing the adjacent part number, return Error.
                    let num = match numeric_string.parse::<i32>() {
                        Ok(num) => num,
                        Err(e) => return Err(e.to_string()),
                    };
                    result.push(num);
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

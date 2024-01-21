use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const TESTFILEPATH: &str = "./test_input.txt";
const FILEPATH: &str = "./input.txt";

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let is_test = args.contains(&"-t".to_string());
    let file_path = match is_test {
        true => TESTFILEPATH,
        false => FILEPATH,
    };
    let debug_print = args.contains(&"-d".to_string());

    let buf = BufReader::new(open_file(file_path));
    let data = BoatRaceData::parse(buf, debug_print).unwrap();

    let viable_time_ranges = data.get_charge_time_ranges(debug_print);
    let mut result = 0;
    for (low, high) in viable_time_ranges {
        if result != 0 {
            result *= high - low + 1;
        } else {
            result = high - low + 1;
        }
        if debug_print {
            println!("({}, {}), range {}", low, high, high - low);
        }
    }

    println!("{}", result);
}

pub enum BoatDataType {
    Time(Vec<u32>),
    Distance(Vec<u32>),
}

pub struct BoatRaceData {
    times: Vec<u32>,
    distances: Vec<u32>,
}
impl BoatRaceData {
    pub fn parse(buf: BufReader<File>, debug_print: bool) -> Result<Self, String> {
        let mut result = Self {
            times: vec![],
            distances: vec![],
        };

        for l in buf.lines() {
            let l = match l {
                Ok(line) => line,
                Err(e) => return Err(e.to_string()),
            };
            let data_group = Self::parse_line(&l, debug_print)?;
            use BoatDataType::*;
            if debug_print {
                match &data_group {
                    Time(d) => println!("Parsed Time: {:?}", d),
                    Distance(d) => println!("Parsed Distance: {:?}", d),
                }
            }
            match data_group {
                Time(data) => result.times = data,
                Distance(data) => result.distances = data,
            }
        }

        if result.distances.len() != result.times.len() {
            return Err("The BoatRaceData times don't have corrosponding distances.".to_string());
        }

        return Ok(result);
    }

    pub fn get_charge_time_ranges(&self, _debug_print: bool) -> Vec<(u32, u32)> {
        let mut result = vec![];
        let calc_lowest = |time: u32, distance: u32| -> Option<u32> {
            let mut mid = time / 2;
            let mut is_too_low = mid * (time - mid) < distance;
            if is_too_low {
                return None;
            }
            let mut high_end = mid;
            let mut low_end = 0;

            while low_end <= high_end {
                mid = low_end + (high_end - low_end) / 2;
                let curr_mid_distance = mid * (time - mid);

                if curr_mid_distance == distance {
                    return Some(mid + 1);
                }
                is_too_low = curr_mid_distance < distance;

                if is_too_low {
                    low_end = mid + 1;
                } else {
                    high_end = mid - 1;
                }
            }

            return Some(if is_too_low { low_end + 1 } else { low_end });
        };

        for i in 0..self.times.len() {
            let time = self.times[i];
            let distance = self.distances[i];

            let lowest = calc_lowest(time, distance);
            if let Some(n) = lowest {
                result.push((n, time - n));
            }
        }

        return result;
    }

    fn parse_line(line: &String, _debug_print: bool) -> Result<BoatDataType, String> {
        let parse_num = |numeric_string: &mut String| -> Result<u32, String> {
            let result = match numeric_string.parse::<u32>() {
                Ok(n) => n,
                Err(e) => return Err(e.to_string()),
            };

            numeric_string.clear();

            return Ok(result);
        };

        let trim_index = match line.find(':') {
            Some(i) => i,
            None => return Err("Line does not contain a Colon!".into()),
        };

        let mut result = vec![];

        let mut numeric_string = "".to_string();
        for c in line[trim_index + 1..line.len()].trim().chars() {
            match c {
                '0'..='9' => {
                    numeric_string.push(c);
                }
                _ => {
                    if numeric_string.len() == 0 {
                        continue;
                    }

                    match parse_num(&mut numeric_string) {
                        Ok(n) => result.push(n),
                        Err(e) => return Err(e),
                    }
                }
            }
        }

        match parse_num(&mut numeric_string) {
            Ok(n) => result.push(n),
            Err(e) => return Err(e),
        }

        return Ok(match line[0..trim_index].len() == 4 {
            true => BoatDataType::Time(result),
            _ => BoatDataType::Distance(result),
        });
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

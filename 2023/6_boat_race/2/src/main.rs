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
        let solutions = high - low + 1;
        if result != 0 {
            result *= solutions;
        } else {
            result = solutions;
        }
        if debug_print {
            println!("({}, {}), range {}", low, high, solutions);
        }
    }

    println!("{}", result);
}

enum BoatDataType {
    Time(Vec<u64>),
    Distance(Vec<u64>),
}

pub struct BoatRaceData {
    times: Vec<u64>,
    distances: Vec<u64>,
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

    pub fn get_charge_time_ranges(&self, debug_print: bool) -> Vec<(u64, u64)> {
        let mut result = vec![];

        for i in 0..self.times.len() {
            let time = self.times[i];
            let distance = self.distances[i];

            if let Some(n) = Self::calc_lowest(time, distance, debug_print) {
                let high_end = time - n;
                if debug_print {
                    println!(
                        "Result for Time {}, Distance {}: ({}, {})",
                        time, distance, n, high_end
                    );
                }
                result.push((n, high_end));
            }
        }

        return result;
    }

    fn calc_lowest(time: u64, distance: u64, debug_print: bool) -> Option<u64> {
        let is_odd = time % 2 != 0;
        let mut mid = if is_odd { (time / 2) + 1 } else { time / 2 };
        let mut is_too_low = mid * (time - mid) <= distance;

        let print_debug = |low_end: u64| {
            let format_calc = |calc_type: &str, t: u64| -> String {
                let offset = time - t;
                return format!("{}: {} * {} = {}", calc_type, t, offset, t * offset);
            };
            let (just_under_low, just_under_high, just_over_low, just_over_high) =
                (low_end - 1, time - low_end + 1, low_end, time - low_end);
            let just_under_low = format_calc("Just Under Low", just_under_low);
            let just_under_high = format_calc("Just Under High", just_under_high);
            let just_over_high = format_calc("Just Over High", just_over_high);
            let just_over_low = format_calc("Just Over Low", just_over_low);

            println!(
                "Calculations for Time {}, Distance {}:\n{}\n{}\n{}\n{}",
                time, distance, just_under_low, just_over_low, just_under_high, just_over_high
            );
        };

        if is_too_low {
            if debug_print {
                let offset = time - mid;
                println!(
                    "No viable times for Time {}, Distance {}: {} * {} = {}",
                    time,
                    distance,
                    mid,
                    offset,
                    mid * offset
                );
            }
            return None;
        }
        let mut high_end = mid;
        let mut low_end = 0;

        while low_end <= high_end {
            mid = low_end + (high_end - low_end) / 2;
            let curr_mid_distance = mid * (time - mid);

            if curr_mid_distance == distance {
                if debug_print {
                    print_debug(mid + 1);
                }
                return Some(mid + 1);
            }
            is_too_low = curr_mid_distance < distance;

            if is_too_low {
                low_end = mid + 1;
            } else {
                high_end = mid - 1;
            }
        }

        if debug_print {
            print_debug(low_end);
        }

        return Some(low_end);
    }

    fn parse_line(line: &String, _debug_print: bool) -> Result<BoatDataType, String> {
        let parse_num = |numeric_string: &mut String| -> Result<u64, String> {
            let result = match numeric_string.parse::<u64>() {
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

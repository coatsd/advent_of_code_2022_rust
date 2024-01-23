use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

const START: &str = "AAA";
const END: &str = "ZZZ";

pub struct Location {
    pos: String,
    fork: (String, String),
}
impl Location {
    pub fn new(pos: String, fork: (String, String)) -> Self {
        return Self { pos, fork };
    }
}
impl Eq for Location {}
impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        let string_cmp = self.pos.cmp(&other.pos);
        if string_cmp == std::cmp::Ordering::Equal {
            return true;
        }
        return false;
    }
}
impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.pos.cmp(&other.pos);
    }
}
impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.pos.cmp(&other.pos));
    }
}
impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos: {}, fork: ({}, {})",
            self.pos, self.fork.0, self.fork.1
        )
    }
}

pub struct Locations(Vec<Location>);
impl Locations {
    pub fn get_value(&self, k: &String) -> Option<&(String, String)> {
        let mut l = 0;
        let mut r = self.0.len() - 1;

        while l <= r {
            let mid = l + (r - l) / 2;

            let string_cmp = (*self.0[mid].pos).cmp(k);

            use std::cmp::Ordering::*;
            match string_cmp {
                Equal => return Some(&self.0[mid].fork),
                Less => l = mid + 1,
                Greater => r = mid - 1,
            }
        }
        return None;
    }
}
impl Display for Locations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "Locations:\n".to_string();
        for i in 0..self.0.len() {
            result = format!("{}\t{}\n", result, self.0[i]);
        }
        write!(f, "{}", result)
    }
}

pub struct Map {
    steps: Vec<bool>,
    locations: Locations,
    start: String,
}
impl Map {
    pub fn parse(buf: BufReader<File>) -> Result<Self, String> {
        let mut steps = vec![];
        let mut locations = vec![];
        let start = START.to_string();

        for (i, l) in buf.lines().enumerate() {
            let l = match l {
                Ok(line) => line,
                Err(e) => return Err(e.to_string()),
            };

            if l.is_empty() {
                continue;
            }

            if i == 0 {
                Self::parse_steps(l, &mut steps)?;
                continue;
            }

            Self::parse_map_line(l, &mut locations)?;
        }

        locations.sort();

        return Ok(Self {
            steps,
            locations: Locations(locations),
            start,
        });
    }
    fn parse_steps(l: String, steps: &mut Vec<bool>) -> Result<(), String> {
        for c in l.trim().chars() {
            match c {
                'L' => steps.push(false),
                'R' => steps.push(true),
                _ => return Err(format!("Invalid character in steps line: {}", c)),
            }
        }
        return Ok(());
    }
    fn parse_map_line(l: String, map: &mut Vec<Location>) -> Result<(), String> {
        let mut line_data = "".to_string();
        for c in l.chars() {
            match c {
                'A'..='Z' => line_data.push(c),
                _ => continue,
            }
        }

        if line_data.len() != 9 {
            return Err(format!("Line was not formatted correctly: {}", l));
        }

        let pos = line_data[0..3].to_string();
        let fork = (line_data[3..6].to_string(), line_data[6..9].to_string());

        map.push(Location::new(pos, fork));

        return Ok(());
    }

    pub fn traverse_map(&self) -> Result<u32, String> {
        let mut result = 0;
        let mut curr_pos: &String = &self.start;
        let mut command_index = 0;

        while curr_pos != END {
            result += 1;
            let command = self.steps[command_index];
            curr_pos = match command {
                true => match self.locations.get_value(curr_pos) {
                    Some(fork) => &fork.1,
                    None => return Err(format!("Could not find position {}", curr_pos)),
                },
                false => match self.locations.get_value(curr_pos) {
                    Some(fork) => &fork.0,
                    None => return Err(format!("Could not find position {}", curr_pos)),
                },
            };

            if command_index < self.steps.len() - 1 {
                command_index += 1;
            } else {
                command_index = 0;
            }
        }

        return Ok(result);
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "Steps:\n\t".to_string();
        for i in 0..self.steps.len() {
            let step = if self.steps[i] { 'R' } else { 'L' };
            result = format!("{}{}", result, step);
        }
        result = format!("{}\n{}", result, self.locations);
        write!(f, "{}", result)
    }
}

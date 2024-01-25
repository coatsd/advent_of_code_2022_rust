use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

const STARTCHAR: char = 'A';
const ENDCHAR: char = 'Z';

#[derive(Clone, Copy)]
pub struct Coord([char; 3]);
impl Coord {
    pub fn parse(set: String) -> Result<Self, String> {
        if set.len() != 3 {
            return Err(format!("String is an invalid Coordinate: {}", set));
        }
        let mut result = [' '; 3];

        for (i, c) in set.chars().enumerate() {
            result[i] = c;
        }

        return Ok(Coord(result));
    }

    pub fn is_start(&self) -> bool {
        return self.0[2] == STARTCHAR;
    }

    pub fn is_end(&self) -> bool {
        return self.0[2] == ENDCHAR;
    }

    pub fn val(&self) -> &[char; 3] {
        return &self.0;
    }
}
impl Eq for Coord {}
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        return true;
    }
}
impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::Equal;
        for i in 0..self.0.len() {
            let char_cmp = self.0[i].cmp(&other.0[i]);
            if char_cmp != Equal {
                return char_cmp;
            }
        }
        return Equal;
    }
}
impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}
impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "".to_string();
        for i in 0..self.0.len() {
            result.push(self.0[i]);
        }
        write!(f, "{}", result)
    }
}

pub struct Location {
    pos: Coord,
    fork: (Coord, Coord),
}
impl Location {
    pub fn new(pos: Coord, fork: (Coord, Coord)) -> Self {
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
    pub fn get_value(&self, k: &Coord) -> Option<&(Coord, Coord)> {
        let mut l = 0;
        let mut r = self.0.len() - 1;

        while l <= r {
            let mid = l + (r - l) / 2;

            let string_cmp = self.0[mid].pos.cmp(k);

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
    start: Vec<Coord>,
}
impl Map {
    pub fn parse(buf: BufReader<File>) -> Result<Self, String> {
        let mut steps = vec![];
        let mut locations = vec![];
        let mut start = vec![];

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

            Self::parse_location(l, &mut locations, &mut start)?;
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
    fn parse_location(
        l: String,
        map: &mut Vec<Location>,
        start: &mut Vec<Coord>,
    ) -> Result<(), String> {
        let mut line_data = "".to_string();
        for c in l.chars() {
            match c {
                'A'..='Z' | '0'..='9' => line_data.push(c),
                _ => continue,
            }
        }

        if line_data.len() != 9 {
            return Err(format!("Line was not formatted correctly: {}", l));
        }

        let pos = Coord::parse(line_data[0..3].to_string())?;
        let fork = (
            Coord::parse(line_data[3..6].to_string())?,
            Coord::parse(line_data[6..9].to_string())?,
        );
        if pos.is_start() {
            start.push(pos);
        }

        map.push(Location::new(pos, fork));

        return Ok(());
    }

    pub fn traverse_map(&self) -> Result<u32, String> {
        let mut result = 0;
        let mut curr_pos = self.start.clone();
        let mut command_index = 0;
        let is_end = |curr_pos: &Vec<Coord>| -> bool {
            for i in 0..curr_pos.len() {
                if !curr_pos[i].is_end() {
                    return false;
                }
            }
            return true;
        };

        while !is_end(&curr_pos) {
            result += 1;
            let command = self.steps[command_index];

            for i in 0..curr_pos.len() {
                curr_pos[i] = match command {
                    true => match self.locations.get_value(&curr_pos[i]) {
                        Some(fork) => fork.1,
                        None => return Err(format!("Could not find position {}", curr_pos[i])),
                    },
                    false => match self.locations.get_value(&curr_pos[i]) {
                        Some(fork) => fork.0,
                        None => return Err(format!("Could not find position {}", curr_pos[i])),
                    },
                };
            }

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
        result = format!("{}\nStart Location:\n\t", result);
        for i in 0..self.start.len() {
            result = format!("{}{}, ", result, self.start[i]);
        }
        result = format!("{}\n{}", result, self.locations);
        write!(f, "{}", result)
    }
}

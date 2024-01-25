use crate::{
    coord::Coord,
    location::{Location, Locations},
};
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

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
        let locations = Locations::new(locations);

        return Ok(Self {
            steps,
            locations,
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

    pub fn traverse_map(&self, debug_print: bool) -> Result<u32, String> {
        let mut path_steps = vec![];

        for i in 0..self.start.len() {
            path_steps.push(self.traverse_one_path(&self.start[i])?);
        }

        let mut path_step_max = 0;
        for i in 0..path_steps.len() {
            if path_steps[i] > path_step_max {
                path_step_max = path_steps[i];
            }
        }

        if debug_print {
            println!(
                "Required Steps for paths: {:?} - Max: {}",
                &path_steps, path_step_max
            );
        }

        let is_required_steps = |result: u32, path_steps: &Vec<u32>| -> bool {
            for i in 0..path_steps.len() {
                if result % path_steps[i] != 0 {
                    return false;
                }
            }
            return true;
        };

        let mut result = path_step_max;
        while !is_required_steps(result, &path_steps) {
            result += path_step_max;
        }

        return Ok(result);
    }
    fn traverse_one_path(&self, coord: &Coord) -> Result<u32, String> {
        let mut result = 0;
        let mut command_index = 0;
        let mut curr_pos = coord.clone();

        while !curr_pos.is_end() {
            result += 1;
            let command = self.steps[command_index];

            curr_pos = match command {
                true => match self.locations.get_value(&curr_pos) {
                    Some(fork) => fork.1,
                    None => return Err(format!("Could not find position {}", curr_pos)),
                },
                false => match self.locations.get_value(&curr_pos) {
                    Some(fork) => fork.0,
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
        result = format!("{}\nStart Location:\n\t", result);
        for i in 0..self.start.len() {
            result = format!("{}{}, ", result, self.start[i]);
        }
        result = format!("{}\n{}", result, self.locations);
        write!(f, "{}", result)
    }
}

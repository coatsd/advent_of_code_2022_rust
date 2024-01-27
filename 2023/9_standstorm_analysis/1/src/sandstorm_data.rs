use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Sandstorm(Vec<i32>);
impl Sandstorm {
    pub fn parse(l: String) -> Result<Self, String> {
        let mut result = vec![];

        let parse_num =
            |numeric_string: &mut String, result: &mut Vec<i32>| -> Result<(), String> {
                match numeric_string.parse() {
                    Ok(n) => result.push(n),
                    Err(e) => return Err(e.to_string()),
                };
                numeric_string.clear();
                return Ok(());
            };

        let mut numeric_string = "".to_string();
        for c in l.chars() {
            match c {
                '-' | '0'..='9' => {
                    numeric_string.push(c);
                }
                _ => parse_num(&mut numeric_string, &mut result)?,
            }
        }
        if !numeric_string.is_empty() {
            parse_num(&mut numeric_string, &mut result)?;
        }

        return Ok(Sandstorm(result));
    }

    pub fn get_value(&self, i: usize) -> i32 {
        return self.0[i];
    }

    pub fn get_next_reading(&self, debug_print: bool) -> Result<i32, String> {
        let mut trends = vec![];

        if self.0.len() > 1 {
            trends.push(self.0[1]);
            trends.push(self.0[1] - self.0[0]);
            if trends[1] != 0 {
                trends.push(0);
            }
        } else {
            return Err(format!(
                "Not enough numbers in sequence to determine trend: {}",
                self
            ));
        }

        let apply_trends = |trends: &mut Vec<i32>| {
            for i in (1..trends.len()).rev() {
                trends[i - 1] += trends[i];
            }
        };

        for i in 1..self.0.len() {
            if self.0[i] == trends[0] {
                apply_trends(&mut trends);
                continue;
            }
            let index = trends.len() - 1;
            trends[index] = self.0[i] - trends[0];
            trends.push(0);
            trends[0] = self.0[i];
            apply_trends(&mut trends);
        }

        if debug_print {
            println!("Projected reading for Sandstorm {} is: {}", self, trends[0]);
        }

        return Ok(trends[0]);
    }
}
impl Display for Sandstorm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "[".to_string();
        for i in 0..self.0.len() {
            let sep = if result.len() != 1 { ", " } else { "" };
            result = format!("{}{}{}", result, sep, self.0[i]);
        }
        result = format!("{}]", result);
        write!(f, "{}", result)
    }
}

pub struct Sandstorms(Vec<Sandstorm>);
impl Sandstorms {
    pub fn parse(buf: BufReader<File>) -> Result<Self, String> {
        let mut result = vec![];

        for l in buf.lines() {
            let l = match l {
                Ok(line) => line,
                Err(e) => return Err(e.to_string()),
            };
            result.push(Sandstorm::parse(l)?);
        }

        return Ok(Self(result));
    }

    pub fn get_value(&self, i: usize) -> &Sandstorm {
        return &self.0[i];
    }

    pub fn get_next_readings(&self, _debug_print: bool) -> Result<Vec<i32>, String> {
        let mut result = vec![];
        for i in 0..self.0.len() {
            result.push(self.0[i].get_next_reading(_debug_print)?);
        }

        return Ok(result);
    }
}
impl Display for Sandstorms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "Sandstorm data:\n".to_string();
        for i in 0..self.0.len() {
            result = format!("{}\t{}\n", result, self.0[i]);
        }
        write!(f, "{}", result)
    }
}

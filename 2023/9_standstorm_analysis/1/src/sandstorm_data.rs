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

    pub fn get_next_reading(&self, debug_print: bool) -> i32 {
        let mut trend_vars = vec![];
        let calc_trend_var = |reading: i32, trend_vars: &Vec<i32>| -> i32 {
            let mut result = reading;
            for t in trend_vars {
                result -= *t;
            }
            return result;
        };
        let apply_trend_vars = |reading: i32, trend_vars: &Vec<i32>| -> i32 {
            let mut result = reading;
            for t in trend_vars {
                result += *t;
            }
            return result;
        };

        for i in 1..self.0.len() {
            if trend_vars.len() == 0 {
                trend_vars.push(self.0[i] - self.0[i - 1]);
                continue;
            }
            let reading_rem = calc_trend_var(self.0[i], &trend_vars);
            if reading_rem != 0 {
                trend_vars.push(reading_rem);
            }
        }

        let result = apply_trend_vars(self.0[self.0.len() - 1], &trend_vars);

        if debug_print {
            println!("Projected reading for Sandstorm {} is: {}", self, result);
        }

        return result;
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

    pub fn get_next_readings(&self, _debug_print: bool) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..self.0.len() {
            result.push(self.0[i].get_next_reading(_debug_print));
        }

        return result;
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

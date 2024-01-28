use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Sandstorm(Vec<i64>);
impl Sandstorm {
    pub fn parse(l: String) -> Result<Self, String> {
        let mut result = vec![];

        let parse_num =
            |numeric_string: &mut String, result: &mut Vec<i64>| -> Result<(), String> {
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

        result.reverse();

        return Ok(Sandstorm(result));
    }

    pub fn get_value(&self, i: usize) -> i64 {
        return self.0[i];
    }

    pub fn get_next_reading(&self, debug_print: bool) -> Result<i64, String> {
        if debug_print {
            println!("Starting reading for Sandstorm: {}", self);
        }

        let print_debug_trends = |trends: &Vec<Vec<i64>>| {
            if debug_print {
                let mut result = format!("Current trends:\n\t{}\n", self);
                for i in 0..trends.len() {
                    result = format!("{}\t{:?}\n", result, trends[i]);
                }
                println!("{}", result);
            }
        };

        let calc_trend = |trend: &Vec<i64>| {
            (1..trend.len())
                .into_iter()
                .map(|i| trend[i] - trend[i - 1])
                .collect::<Vec<i64>>()
        };
        let contains_zeros = |trend: &Vec<i64>| -> bool {
            for i in 0..trend.len() {
                if trend[i] != 0 {
                    return false;
                }
            }
            return true;
        };
        let beg_trend = calc_trend(&self.0);
        let mut trends = vec![beg_trend];
        print_debug_trends(&trends);
        while !contains_zeros(&trends[trends.len() - 1]) {
            trends.push(calc_trend(&trends[trends.len() - 1]));
            print_debug_trends(&trends);
        }

        let mut end_trends = vec![];
        for trend in trends {
            end_trends.push(trend[trend.len() - 1]);
        }

        if debug_print {
            println!("Adding resulting ends: {:?}", end_trends);
        }
        let mut result = self.0[self.0.len() - 1];
        for i in 0..end_trends.len() {
            result += end_trends[i];
        }

        return Ok(result);
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

    pub fn get_next_readings(&self, _debug_print: bool) -> Result<Vec<i64>, String> {
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

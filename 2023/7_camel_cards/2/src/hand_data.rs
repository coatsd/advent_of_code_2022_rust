use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::hand::Hand;

pub struct HandData(pub Vec<Hand>);
impl HandData {
    pub fn parse(buf: BufReader<File>) -> Result<Self, String> {
        let mut result = vec![];

        for l in buf.lines() {
            let l = match l {
                Ok(s) => s,
                Err(e) => return Err(e.to_string()),
            };

            result.push(match Hand::parse(l) {
                Ok(h) => h,
                Err(e) => return Err(e),
            });
        }

        return Ok(HandData(result));
    }

    pub fn sort_by_rank(&mut self) {
        return self.0.sort();
    }
}

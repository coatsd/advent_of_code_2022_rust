use crate::coord::Coord;
use std::fmt::Display;

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
    pub fn new(items: Vec<Location>) -> Self {
        return Locations(items);
    }

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

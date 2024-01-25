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
impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "".to_string();
        for i in 0..self.0.len() {
            result.push(self.0[i]);
        }
        write!(f, "{}", result)
    }
}

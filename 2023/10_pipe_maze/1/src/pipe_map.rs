use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::Index,
};

pub struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        return Self { x, y };
    }

    pub fn x(&self) -> usize {
        return self.x;
    }

    pub fn y(&self) -> usize {
        return self.y;
    }
}
impl Eq for Coord {}
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        return self.x() == other.x() && self.y() == other.y();
    }
}
impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

pub enum Tile {
    VPipe,
    HPipe,
    NEPipe,
    NWPipe,
    SEPipe,
    SWPipe,
    Start,
    Ground,
}
impl Tile {
    pub fn parse(c: char) -> Self {
        use Tile::*;
        return match c {
            '|' => VPipe,
            '-' => HPipe,
            'L' => NEPipe,
            'J' => NWPipe,
            '7' => SWPipe,
            'F' => SEPipe,
            'S' => Start,
            _ => Ground,
        };
    }

    pub fn get_char(&self) -> char {
        use Tile::*;
        return match self {
            VPipe => '|',
            HPipe => '-',
            NEPipe => 'L',
            NWPipe => 'J',
            SWPipe => '7',
            SEPipe => 'F',
            Start => 'S',
            _ => '.',
        };
    }

    pub fn is_connected(&self, other: &Self, self_coord: &Coord, other_coord: &Coord) -> bool {
        let x_diff = (self_coord.x() as i32) - (other_coord.x() as i32);
        let y_diff = (self_coord.y() as i32) - (other_coord.y() as i32);

        use Tile::*;
        return match (self, other, x_diff, y_diff) {
            // Connection combinations for:
            // Left of other
            (Start | SEPipe | HPipe | NEPipe, Start | NWPipe | HPipe | SWPipe, -1, 0)
            // Above other
            | (Start | SWPipe | VPipe | SEPipe, Start | NWPipe | VPipe | NEPipe, 0, -1)
            // Right of other
            | (Start | NWPipe | HPipe | SWPipe, Start | SEPipe | HPipe | NEPipe, 1, 0)
            // Below other
            | (Start | NWPipe | VPipe | NEPipe, Start | SWPipe | VPipe | SEPipe, 0, 1) => true,
            // All else are not connected.
            _ => false,
        };
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

pub struct PipeMap {
    map: Vec<Vec<Tile>>,
    pub start: Coord,
}
impl PipeMap {
    pub fn parse(buf: BufReader<File>) -> Result<Self, String> {
        let mut map = vec![];
        let (mut x, mut y) = (0, 0);
        for (i, l) in buf.lines().enumerate() {
            let l = match l {
                Ok(line) => line,
                Err(e) => return Err(e.to_string()),
            };
            let tiles = l
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    let tile = Tile::parse(c);
                    if let Tile::Start = tile {
                        (x, y) = (j, i);
                    }
                    return tile;
                })
                .collect::<Vec<Tile>>();
            map.push(tiles);
        }

        return Ok(Self {
            map,
            start: Coord { x, y },
        });
    }

    pub fn is_connected_tile(&self, source: &Coord, dest: &Coord) -> bool {
        if !self.is_in_bound(&source) || !self.is_in_bound(&dest) {
            return false;
        }

        return self[source].is_connected(&self[dest], &source, &dest);
    }

    pub fn is_in_bound(&self, pos: &Coord) -> bool {
        return pos.y() < self.map.len() && pos.x() < self.map[0].len();
    }
}
impl Index<&Coord> for PipeMap {
    type Output = Tile;
    fn index(&self, index: &Coord) -> &Self::Output {
        return &self.map[index.y()][index.x()];
    }
}
impl Display for PipeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("Starting Coord: {}\n", self.start);
        for i in 0..self.map.len() {
            let mut l = "".to_string();
            for j in 0..self.map[i].len() {
                l.push(self.map[i][j].get_char());
            }
            l.push('\n');
            result = format!("{}{}", result, l);
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use crate::pipe_map::{Coord, PipeMap, Tile};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    pub fn open_file<P>(path: P) -> File
    where
        P: AsRef<Path> + std::fmt::Display,
    {
        let file = std::fs::File::open(&path);
        match file {
            Ok(file) => file,
            Err(e) => panic!("Could not open file {}: {}", path, e),
        }
    }

    #[test]
    fn test_pipe_map_parse() {
        let file_path = "./test_input.txt";
        let buf = BufReader::new(open_file(file_path));
        let pipe_map = PipeMap::parse(buf).unwrap();
        let buf = BufReader::new(open_file(file_path));
        let mut cmp = "Starting Coord: (2, 0)\n".to_string();
        for l in buf.lines() {
            let l = match l {
                Ok(l) => l,
                Err(e) => panic!("There was a problem parsing line: {}", e.to_string()),
            };
            cmp = format!("{}{}\n", cmp, l);
        }
        assert_eq!(pipe_map.to_string(), cmp);
    }

    #[test]
    fn test_tile_is_connected() {
        use Tile::*;
        assert_eq!(
            true,
            HPipe.is_connected(&HPipe, &Coord::new(0, 0), &Coord::new(1, 0))
        );
        assert_eq!(
            false,
            HPipe.is_connected(&HPipe, &Coord::new(0, 0), &Coord::new(0, 1))
        );
        assert_eq!(
            true,
            VPipe.is_connected(&SWPipe, &Coord::new(0, 1), &Coord::new(0, 0))
        );
        assert_eq!(
            false,
            VPipe.is_connected(&NWPipe, &Coord::new(0, 0), &Coord::new(1, 0))
        );
    }

    #[test]
    fn test_pipe_map_connected_tile() {
        let file_path = "./test_input.txt";
        let buf = BufReader::new(open_file(file_path));
        let pipe_map = PipeMap::parse(buf).unwrap();
        assert_eq!(
            false,
            pipe_map.is_connected_tile(&Coord::new(99, 0), &Coord::new(98, 0))
        );
        assert_eq!(
            false,
            pipe_map.is_connected_tile(&Coord::new(2, 0), &Coord::new(1, 0))
        );
        assert_eq!(
            true,
            pipe_map.is_connected_tile(&Coord::new(2, 0), &Coord::new(3, 0))
        );
        assert_eq!(
            true,
            pipe_map.is_connected_tile(&Coord::new(1, 1), &Coord::new(1, 2))
        );
        assert_eq!(
            false,
            pipe_map.is_connected_tile(&Coord::new(1, 2), &Coord::new(1, 3))
        );
    }
}

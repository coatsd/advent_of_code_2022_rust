use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
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
            '7' => SEPipe,
            'F' => SWPipe,
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
            SEPipe => '7',
            SWPipe => 'F',
            Start => 'S',
            _ => '.',
        };
    }

    pub fn is_connected(&self, other: &Self, self_coord: &Coord, other_coord: &Coord) -> bool {
        let x_diff = self_coord.x() as i32 - other_coord.x() as i32;
        let y_diff = self_coord.y() as i32 - other_coord.y() as i32;

        use Tile::*;
        return match (self, other, x_diff, y_diff) {
            // If either are Ground, false.
            (Ground, _, _, _)
            | (_, Ground, _, _) => false,
            // If either are Start, true.
            (Start, _, _, _)
            | (_, Start, _, _) => true,
            // Connection combinations for:
            // Left of other
            (SEPipe | HPipe | NEPipe, NWPipe | HPipe | SWPipe, 1, 0)
            // Above other
            | (SWPipe | VPipe | SEPipe, NWPipe | VPipe | NEPipe, 0, 1)
            // Right of other
            | (NWPipe | HPipe | SWPipe, SEPipe | HPipe | NEPipe, -1, 0)
            // Below other
            | (NWPipe | VPipe | NEPipe, SWPipe | VPipe | SEPipe, 0, 1) => true,
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
                        (x, y) = (i, j);
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
        if !self.is_in_bound(&dest) {
            return false;
        }

        return self
            .get_value(&source)
            .is_connected(&self.get_value(&dest), &source, &dest);
    }

    pub fn is_in_bound(&self, pos: &Coord) -> bool {
        return pos.y() < self.map.len() && pos.x() < self.map[0].len();
    }

    fn get_value(&self, coord: &Coord) -> &Tile {
        return &self.map[coord.y()][coord.x()];
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

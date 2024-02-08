use std::fmt::Display;

use crate::pipe_map::{Coord, PipeMap};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    pub fn inverse(&self) -> Self {
        use Direction::*;
        return match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        };
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        let result = match self {
            Left => "Left",
            Up => "Up",
            Right => "Right",
            Down => "Down",
        };
        write!(f, "{}", result)
    }
}

struct AdjacentTiles([Option<Coord>; 4]);
impl AdjacentTiles {
    pub fn get_by_dir(&self, dir: &Direction) -> &Option<Coord> {
        return &self.0[Self::get_index_by_dir(dir)];
    }

    pub fn get_index_by_dir(dir: &Direction) -> usize {
        use Direction::*;
        return match dir {
            Left => 0,
            Up => 1,
            Right => 2,
            Down => 3,
        };
    }

    pub fn get_dir_by_index(index: usize) -> Result<Direction, String> {
        use Direction::*;
        return Ok(match index {
            0 => Left,
            1 => Up,
            2 => Right,
            3 => Down,
            _ => return Err(format!("index {} is out of range of AdjacentTiles.", index)),
        });
    }

    pub fn parse(pos: &Coord, pipe_map: &PipeMap) -> Self {
        const COORDOPTIONINIT: Option<Coord> = None;
        let mut result = [COORDOPTIONINIT; 4];

        let mod_num = |coord: usize, offset: i8| -> Option<usize> {
            return match offset {
                -1 => coord.checked_sub(1),
                _ => coord.checked_add(offset as usize),
            };
        };

        for (i, n) in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter().enumerate() {
            let x_y_set = (mod_num(pos.x(), n.0), mod_num(pos.y(), n.1));
            if let (Some(x), Some(y)) = x_y_set {
                let new_coord = Coord::new(x, y);
                if pipe_map.is_in_bound(&new_coord) && pipe_map.is_connected_tile(pos, &new_coord) {
                    result[i] = Some(new_coord);
                }
            }
        }

        return AdjacentTiles(result);
    }
}

enum NextMoveParseError {
    NoValidNextMoves,
}

struct NextMove {
    next: Coord,
    last_dir: Direction,
}
impl NextMove {
    pub fn parse(
        adjacent_tiles: &AdjacentTiles,
        last_dir: &Direction,
    ) -> Result<Self, NextMoveParseError> {
        use Direction::*;
        for d in [Left, Up, Right, Down] {
            if *last_dir == d {
                continue;
            }
            if let Some(c) = adjacent_tiles.get_by_dir(&d) {
                let last_dir = d.inverse();
                let next = Coord::new(c.x(), c.y());
                return Ok(Self { next, last_dir });
            }
        }
        return Err(NextMoveParseError::NoValidNextMoves);
    }
}

pub struct TileCrawler {
    start: Coord,
}
impl TileCrawler {
    pub fn new(start: &Coord) -> Self {
        return Self {
            start: Coord::new(start.x(), start.y()),
        };
    }

    pub fn get_longest_crawl_path(&self, pipe_map: &PipeMap, debug_print: bool) -> u32 {
        let adjacent_start_coords = AdjacentTiles::parse(&self.start, &pipe_map);

        let mut crawl_results = [0u32; 4];
        for i in 0..crawl_results.len() {
            if crawl_results[i] != 0 {
                continue;
            }
            let (steps, reentry) = self.crawl(
                &pipe_map,
                &adjacent_start_coords,
                AdjacentTiles::get_dir_by_index(i).unwrap(),
                debug_print,
            );
            crawl_results[i] = steps;
            crawl_results[AdjacentTiles::get_index_by_dir(&reentry)] = steps;
        }

        let mut result = 0;
        for r in crawl_results.iter() {
            if *r == 0 {
                continue;
            }
            if *r > result {
                result = *r;
            }
        }

        result = (result / 2) + (result % 2);
        return result;
    }
    fn crawl(
        &self,
        pipe_map: &PipeMap,
        adjacent_start_coords: &AdjacentTiles,
        dir: Direction,
        _debug_print: bool,
    ) -> (u32, Direction) {
        let (mut curr_tile, mut from) = match adjacent_start_coords.get_by_dir(&dir) {
            Some(c) => (Coord::new(c.x(), c.y()), dir.inverse()),
            None => return (0, dir.clone()),
        };
        let mut steps = 0;

        while curr_tile != self.start {
            let adjacent_tiles = AdjacentTiles::parse(&curr_tile, &pipe_map);
            let next_move = match NextMove::parse(&adjacent_tiles, &from) {
                Ok(n) => n,
                Err(e) => match e {
                    NextMoveParseError::NoValidNextMoves => {
                        panic!("No next moves from {curr_tile}")
                    }
                },
            };
            curr_tile = next_move.next;
            from = next_move.last_dir;
            steps += 1;
        }

        return (steps, from);
    }
}

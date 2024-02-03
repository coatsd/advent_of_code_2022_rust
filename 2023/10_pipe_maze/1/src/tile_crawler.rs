use crate::pipe_map::{Coord, PipeMap};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
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

    pub fn get_longest_crawl_path(&self, pipe_map: &PipeMap) -> u32 {
        const COORDOPTIONINIT: Option<Coord> = None;
        let mut direction_coords = [COORDOPTIONINIT; 4];
        use Direction::*;
        let directions = [Left, Up, Right, Down];

        for i in 0..directions.len() {
            direction_coords[i] =
                Self::fetch_coord_if_connected(&self.start, &pipe_map, &directions[i]);
        }

        let mut crawl_results = [0u32; 4];
        for i in 0..direction_coords.len() {
            if crawl_results[i] != 0 {
                continue;
            }
            let (steps, reentry) = self.crawl(&pipe_map, &direction_coords, &directions[i]);
            crawl_results[i] = steps;
            match reentry {
                Left => crawl_results[0] = steps,
                Up => crawl_results[1] = steps,
                Right => crawl_results[2] = steps,
                Down => crawl_results[3] = steps,
            }
        }

        let mut result = 0;
        for r in crawl_results.iter() {
            if *r > result {
                result = *r;
            }
        }

        if result % 2 != 0 {
            result /= 2 + 1;
        } else {
            result /= 2;
        }

        return result;
    }
    fn crawl(
        &self,
        pipe_map: &PipeMap,
        direction_coords: &[Option<Coord>; 4],
        dir: &Direction,
    ) -> (u32, Direction) {
        use Direction::*;
        let get_dir_coord_by_dir = |dir: &Direction| -> &Option<Coord> {
            return &direction_coords[match dir {
                Left => 0,
                Up => 1,
                Right => 2,
                Down => 3,
            }];
        };
        let next = get_dir_coord_by_dir(&dir);
        let next = match next {
            Some(c) => c,
            None => return (0, dir.clone()),
        };

        let get_inverse_direction = |dir: &Direction| -> Direction {
            return match dir {
                Left => Right,
                Right => Left,
                Up => Down,
                Down => Up,
            };
        };

        let mut curr_pos = Coord::new(next.x(), next.y());
        let directions = [Left, Up, Right, Down];
        let mut steps = 0;
        let mut from = Left;
        while curr_pos != self.start {
            for i in 0..directions.len() {
                let to = directions[i];
                if to == from {
                    continue;
                }
                (curr_pos, steps, from) =
                    match Self::fetch_coord_if_connected(&curr_pos, &pipe_map, &to) {
                        Some(c) => (c, steps + 1, get_inverse_direction(&to)),
                        None => continue,
                    };
                break;
            }
        }

        return (steps, from);
    }
    fn fetch_coord_if_connected(pos: &Coord, pipe_map: &PipeMap, dir: &Direction) -> Option<Coord> {
        use Direction::*;
        let (mod_x, mod_y) = match dir {
            Left => (-1, 0),
            Right => (1, 0),
            Up => (0, -1),
            Down => (0, 1),
        };
        let mod_num = |coord: usize, offset: i8| -> Option<usize> {
            return match offset {
                -1 => coord.checked_sub(1),
                _ => coord.checked_add(offset as usize),
            };
        };

        let new_coord = match (mod_num(pos.x(), mod_x), mod_num(pos.y(), mod_y)) {
            (Some(x), Some(y)) => Some(Coord::new(x, y)),
            _ => None,
        };
        if let Some(c) = new_coord {
            if pipe_map.is_in_bound(&c) && pipe_map.is_connected_tile(&pos, &c) {
                return Some(c);
            }
        }
        return None;
    }
}

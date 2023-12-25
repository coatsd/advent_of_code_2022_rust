use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";
const REDLIMIT: i32 = 12;
const GREENLIMIT: i32 = 13;
const BLUELIMIT: i32 = 14;

fn main() {
    let buf = BufReader::new(open_file("./input.txt"));
    let mut result: i32 = 0;
    for game in buf.lines() {
        if let Ok(game) = game {
            if game.len() == 0 {
                continue;
            }
            result += match parse_game_id_if_possible(game).unwrap() {
                Some(n) => n,
                None => 0,
            };
        }
    }
    println!("{}", result);
}

fn parse_game_id_if_possible(game: String) -> Result<Option<i32>, String> {
    let (game_id, game_rounds) = if let Some(i) = game.find(':') {
        let game_id = game[0..i]
            .trim()
            .replace("Game ", "")
            .parse::<i32>()
            .unwrap();
        (game_id, &game[i + 1..game.len()])
    } else {
        panic!(
            "Game does not contain a semicolon delimiter! game: {}",
            game
        );
    };

    if game_is_possible(game_rounds)? {
        return Ok(Some(game_id));
    }
    return Ok(None);
}

fn game_is_possible(game_rounds: &str) -> Result<bool, String> {
    let game_round_vec = game_rounds.split(';').collect::<Vec<&str>>();
    return test_game_rounds(game_round_vec);
}

fn test_game_rounds(rounds: Vec<&str>) -> Result<bool, String> {
    for round in rounds.iter() {
        let cube_groups = round.split(',').collect::<Vec<&str>>();
        for cube_group in cube_groups {
            for color in [RED, GREEN, BLUE] {
                if cube_group.contains(color) {
                    let cube_count = if let Ok(num) =
                        cube_group.replace(color, "").trim().parse::<i32>()
                    {
                        num
                    } else {
                        panic!("Algorithm for cube_count parsing is not functioning correctly with input \"{:?}\"", rounds);
                    };
                    match color {
                        "red" => {
                            if cube_count > REDLIMIT {
                                return Ok(false);
                            }
                        }
                        "green" => {
                            if cube_count > GREENLIMIT {
                                return Ok(false);
                            }
                        }
                        "blue" => {
                            if cube_count > BLUELIMIT {
                                return Ok(false);
                            }
                        }
                        _ => unreachable!(
                            "Colors should only include \"red\", \"green\", or \"blue\"!"
                        ),
                    }
                    break;
                }
            }
        }
    }

    return Ok(true);
}

fn open_file<P>(path: P) -> File
where
    P: AsRef<Path> + std::fmt::Display,
{
    let file = std::fs::File::open(&path);
    match file {
        Ok(file) => file,
        Err(e) => panic!("Could not open file {}: {}", path, e),
    }
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

fn main() {
    let buf = BufReader::new(open_file("./input.txt"));
    let mut result: i32 = 0;
    for game in buf.lines() {
        if let Ok(game) = game {
            if game.len() == 0 {
                continue;
            }
            result += parse_game_power_value(game).unwrap();
        }
    }
    println!("{}", result);
}

fn parse_game_power_value(game: String) -> Result<i32, String> {
    let game_rounds = if let Some(i) = game.find(':') {
        &game[i + 1..game.len()]
    } else {
        panic!(
            "Game does not contain a semicolon delimiter! game: {}",
            game
        );
    };

    return get_round_result(game_rounds);
}

fn get_round_result(game_rounds: &str) -> Result<i32, String> {
    let game_round_vec = game_rounds.split(';').collect::<Vec<&str>>();
    let (red_count, green_count, blue_count) = get_game_rgb_values(game_round_vec)?;
    return Ok(red_count * green_count * blue_count);
}

fn get_game_rgb_values(rounds: Vec<&str>) -> Result<(i32, i32, i32), String> {
    let (mut red_count, mut green_count, mut blue_count) = (0, 0, 0);

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
                            if cube_count > red_count {
                                red_count = cube_count;
                            }
                        }
                        "green" => {
                            if cube_count > green_count {
                                green_count = cube_count;
                            }
                        }
                        "blue" => {
                            if cube_count > blue_count {
                                blue_count = cube_count;
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

    return Ok((red_count, green_count, blue_count));
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

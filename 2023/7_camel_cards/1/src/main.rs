use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use camel_cards::hand_data::HandData;

const TESTFILEPATH: &str = "./test_input.txt";
const FILEPATH: &str = "./input.txt";

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let is_test = args.contains(&"-t".to_string());
    let file_path = match is_test {
        true => TESTFILEPATH,
        false => FILEPATH,
    };
    let _debug_print = args.contains(&"-d".to_string());

    let buf = BufReader::new(open_file(file_path));
    let mut hand_data = HandData::parse(buf).unwrap();
    hand_data.sort_by_rank();

    let mut result = 0;
    for i in 0..hand_data.0.len() {
        let rank = (i + 1) as u32;
        result += hand_data.0[i].calc_winnings(rank);
    }

    println!("{}", result);
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

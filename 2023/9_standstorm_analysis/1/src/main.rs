use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use sandstorm_analysis::sandstorm_data::Sandstorms;

const TESTFILEPATH: &str = "./test_input.txt";
const FILEPATH: &str = "./input.txt";

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let is_test = args.contains(&"-t".to_string());
    let file_path = match is_test {
        true => TESTFILEPATH,
        false => FILEPATH,
    };
    let debug_print = args.contains(&"-d".to_string());

    let buf = BufReader::new(open_file(file_path));
    let sandstorm_data = Sandstorms::parse(buf).unwrap();

    if debug_print {
        println!("{}", sandstorm_data);
    }

    let sandstorm_data_next_readings = sandstorm_data.get_next_readings(debug_print);

    let mut result = 0;
    for reading in sandstorm_data_next_readings {
        result += reading;
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

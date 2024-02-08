use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use pipe_maze::{pipe_map::PipeMap, tile_crawler::TileCrawler};

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
    let pipe_map = PipeMap::parse(buf).unwrap();
    if debug_print {
        println!("Parsed PipeMap:\n{}", pipe_map);
    }

    let crawler = TileCrawler::new(&pipe_map.start);
    let result = crawler.get_longest_crawl_path(&pipe_map, debug_print);

    println!("{}", result);
}

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

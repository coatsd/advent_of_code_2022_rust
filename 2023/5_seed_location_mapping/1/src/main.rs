use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const TESTFILEPATH: &str = "./test_input.txt";
const FILEPATH: &str = "./input.txt";

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let is_test = args.contains(&"-t".to_string());
    let file_path = match is_test {
        true => TESTFILEPATH,
        _ => FILEPATH,
    };
    let debug_print = args.contains(&"-d".to_string());

    let buf = BufReader::new(open_file(file_path));
    let seed_data = SeedData::parse(buf, debug_print).unwrap();
    let seed_locs = seed_data.get_seed_location_data(debug_print).unwrap();

    let mut lowest = u32::MAX;
    for [_, loc] in seed_locs {
        if loc < lowest {
            lowest = loc;
        }
    }

    println!("{}", lowest);
}

pub struct SeedData {
    seeds: Vec<u32>,
    dest_source_cats: [Vec<[u32; 3]>; 7],
}
impl SeedData {
    pub fn parse(buf: BufReader<File>, debug_print: bool) -> Result<Self, String> {
        let mut result = Self::new();
        let mut dest_source_index = -1;

        for (i, l) in buf.lines().enumerate() {
            match l {
                Ok(l) => {
                    if l.len() == 0 {
                        dest_source_index += 1;
                        continue;
                    }

                    if i == 0 {
                        result.seeds = match Self::parse_seeds(l) {
                            Ok(seeds) => seeds,
                            Err(e) => return Err(e),
                        };
                        continue;
                    }

                    if let Err(e) = result.parse_dest_source(l, dest_source_index as usize) {
                        return Err(e);
                    }
                }
                Err(e) => return Err(e.to_string()),
            }
        }
        if debug_print {
            println!("{}", result);
        }

        return Ok(result);
    }

    pub fn get_seed_location_data(&self, debug_print: bool) -> Result<Vec<[u32; 2]>, String> {
        if self.seeds.len() == 0 {
            return Err("Seeds have not been parsed! get_seed_location_data failed.".to_string());
        }
        if self.dest_source_cats.len() == 0 {
            return Err(
                "List of dest_sources have not been parsed! get_seed_location_data failed."
                    .to_string(),
            );
        }

        if debug_print {
            println!("Starting get_seed_location_data...");
        }

        let mut result = vec![];

        for seed in self.seeds.iter() {
            let mut seed_loc = [*seed, *seed];
            for (j, dest_source_cat) in self.dest_source_cats.iter().enumerate() {
                if let Some(d) =
                    Self::parse_dest_if_mentioned(seed_loc[1], dest_source_cat, debug_print)
                {
                    seed_loc[1] = d;
                }
                if debug_print {
                    println!("Seed {}, Category {}: Dest = {}", seed, j, seed_loc[1]);
                }
            }
            if debug_print {
                println!("found location {} for seed {}", seed_loc[1], seed_loc[0]);
            }
            result.push(seed_loc);
        }

        return Ok(result);
    }

    fn parse_dest_if_mentioned(
        source: u32,
        dest_source_cat: &Vec<[u32; 3]>,
        debug_print: bool,
    ) -> Option<u32> {
        let mut lowest_dest_source_cat_val = None;
        for dest_source in dest_source_cat.iter() {
            if debug_print {
                println!(
                    "Checking if Source {} is contained in range {}+{}",
                    source, dest_source[1], dest_source[2],
                );
            }
            let greater_than_upper_bound = match dest_source[1].checked_add(dest_source[2]) {
                Some(n) => source > n,
                None => false,
            };
            if source < dest_source[1] || greater_than_upper_bound {
                continue;
            }
            let current_cat_val = source - dest_source[1] + dest_source[0];
            if let Some(lowest) = lowest_dest_source_cat_val {
                if lowest < current_cat_val {
                    lowest_dest_source_cat_val = Some(current_cat_val);
                }
            } else {
                lowest_dest_source_cat_val = Some(current_cat_val);
            }
        }
        return lowest_dest_source_cat_val;
    }

    fn new() -> Self {
        return Self {
            seeds: Vec::new(),
            dest_source_cats: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
        };
    }

    fn parse_seeds(l: String) -> Result<Vec<u32>, String> {
        if l.len() == 0 {
            return Ok(vec![]);
        }

        let mut result = vec![];
        let mut numeric_string = "".to_string();

        let seed_trim_index = match l.find(':') {
            Some(i) => i,
            None => return Err(format!("Line contains no colon:\n\t{}", l)),
        };

        let l = l[seed_trim_index..l.len()].trim();

        for (i, c) in l.chars().enumerate() {
            match c {
                '0'..='9' => {
                    numeric_string.push(c);
                    if i + 1 == l.len() {
                        match numeric_string.parse::<u32>() {
                            Ok(n) => result.push(n),
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                }
                _ => {
                    if numeric_string.len() == 0 {
                        continue;
                    }
                    match numeric_string.parse::<u32>() {
                        Ok(n) => result.push(n),
                        Err(e) => return Err(e.to_string()),
                    }
                    numeric_string.clear();
                }
            }
        }

        return Ok(result);
    }

    fn parse_dest_source(&mut self, l: String, dest_source_index: usize) -> Result<(), String> {
        if dest_source_index > 6 {
            return Err("Cannot pass an index higher than 6 into parse_dest_source".to_string());
        }
        if l.len() == 0 {
            return Ok(());
        }

        let mut result: [u32; 3] = [0, 0, 0];
        let mut result_index = 0;
        let mut numeric_string = "".to_string();

        for c in l.trim().chars() {
            match c {
                '0'..='9' => numeric_string.push(c),
                ' ' => {
                    match numeric_string.parse::<u32>() {
                        Ok(n) => result[result_index] = n,
                        Err(e) => return Err(e.to_string()),
                    };
                    result_index += 1;
                    numeric_string.clear();
                }
                _ => return Ok(()),
            }
        }

        if numeric_string.len() != 0 {
            match numeric_string.parse::<u32>() {
                Ok(n) => result[2] = n,
                Err(e) => return Err(e.to_string()),
            }
        }

        self.dest_source_cats[dest_source_index].push(result);
        return Ok(());
    }
}
impl std::fmt::Display for SeedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seeds_to_string = || -> String {
            let mut seed_string = "Seeds: ".to_string();
            for seed in self.seeds.as_slice() {
                seed_string = format!("{} {}", seed_string, seed);
            }

            return format!("{}\n", seed_string);
        };
        let cat_to_string = |i: usize| -> String {
            let cat = &self.dest_source_cats[i];

            let mut cat_string = "".to_string();
            for ds in cat {
                cat_string = format!("{}\t{} {} {}\n", cat_string, ds[0], ds[1], ds[2]);
            }

            return format!("\tCategory {} Source-Dests:\n{}", i + 1, cat_string);
        };

        let result = format!(
            "{}Categories:\n{}",
            seeds_to_string(),
            (0..7).map(|i| cat_to_string(i)).collect::<String>()
        );

        write!(f, "{}", result)
    }
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

#[cfg(test)]
mod tests {
    use crate::{open_file, SeedData, TESTFILEPATH};
    use std::fmt::Debug;
    use std::io::BufReader;

    #[test]
    fn test_seed_data_parse() {
        let buf = BufReader::new(open_file(TESTFILEPATH));
        let SeedData {
            seeds,
            dest_source_cats:
                [seed_soils, soil_ferts, fert_waters, water_lights, light_temps, temp_humids, humid_locs],
        } = SeedData::parse(buf, false).unwrap();

        let test_seeds = SeedVec(vec![79, 14, 55, 13]);
        let test_seed_soils = SourceDestVec(vec![[50, 98, 2], [52, 50, 48]]);
        let test_soil_ferts = SourceDestVec(vec![[0, 15, 37], [37, 52, 2], [39, 0, 15]]);
        let test_fert_waters =
            SourceDestVec(vec![[49, 53, 8], [0, 11, 42], [42, 0, 7], [57, 7, 4]]);
        let test_water_lights = SourceDestVec(vec![[88, 18, 7], [18, 25, 70]]);
        let test_light_temps = SourceDestVec(vec![[45, 77, 23], [81, 45, 19], [68, 64, 13]]);
        let test_temp_humids = SourceDestVec(vec![[0, 69, 1], [1, 0, 69]]);
        let test_humid_locs = SourceDestVec(vec![[60, 56, 37], [56, 93, 4]]);

        assert_eq!(SeedVec(seeds), test_seeds);
        assert_eq!(SourceDestVec(seed_soils), test_seed_soils);
        assert_eq!(SourceDestVec(soil_ferts), test_soil_ferts);
        assert_eq!(SourceDestVec(fert_waters), test_fert_waters);
        assert_eq!(SourceDestVec(water_lights), test_water_lights);
        assert_eq!(SourceDestVec(light_temps), test_light_temps);
        assert_eq!(SourceDestVec(temp_humids), test_temp_humids);
        assert_eq!(SourceDestVec(humid_locs), test_humid_locs);
    }

    #[test]
    fn test_seed_data_get_seed_loc_data() {
        let buf = BufReader::new(open_file(TESTFILEPATH));
        let seed_data = SeedData::parse(buf, false).unwrap();
        let seed_locs = SeedLocVec(seed_data.get_seed_location_data(false).unwrap());
        let test_seed_locs = SeedLocVec(vec![[79, 82], [14, 43], [55, 86], [13, 35]]);

        assert_eq!(seed_locs, test_seed_locs);
    }

    #[test]
    fn test_input() {
        let buf = BufReader::new(open_file(TESTFILEPATH));
        let seed_data = SeedData::parse(buf, false).unwrap();
        let seed_locs = seed_data.get_seed_location_data(false).unwrap();

        let mut lowest = u32::MAX;
        for seed_loc in seed_locs {
            if seed_loc[1] < lowest {
                lowest = seed_loc[1];
            }
        }

        assert_eq!(lowest, 35);
    }

    #[derive(Debug)]
    struct SeedVec(Vec<u32>);
    impl std::cmp::Eq for SeedVec {}
    impl std::cmp::PartialEq for SeedVec {
        fn eq(&self, other: &Self) -> bool {
            for i in 0..self.0.len() {
                if i >= other.0.len() {
                    return false;
                }
                if self.0[i] != other.0[i] {
                    return false;
                }
            }
            return true;
        }
    }

    #[derive(Debug)]
    struct SourceDestVec(Vec<[u32; 3]>);
    impl std::cmp::Eq for SourceDestVec {}
    impl std::cmp::PartialEq for SourceDestVec {
        fn eq(&self, other: &Self) -> bool {
            for i in 0..self.0.len() {
                if i >= other.0.len() {
                    return false;
                }
                if self.0[i][0] != other.0[i][0]
                    || self.0[i][1] != other.0[i][1]
                    || self.0[i][2] != other.0[i][2]
                {
                    return false;
                }
            }
            return true;
        }
    }

    #[derive(Debug)]
    struct SeedLocVec(Vec<[u32; 2]>);
    impl std::cmp::Eq for SeedLocVec {}
    impl std::cmp::PartialEq for SeedLocVec {
        fn eq(&self, other: &Self) -> bool {
            for i in 0..self.0.len() {
                if i >= other.0.len() {
                    return false;
                }
                if self.0[i][0] != other.0[i][0] || self.0[i][1] != other.0[i][1] {
                    return false;
                }
            }
            return true;
        }
    }
}

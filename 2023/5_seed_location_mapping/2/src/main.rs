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
        false => FILEPATH,
    };
    let debug_print = args.contains(&"-d".to_string());

    let buf = BufReader::new(open_file(file_path));
    let seed_data = SeedData::parse(buf, debug_print).unwrap();
    let lowest = seed_data.get_lowest_location(debug_print).unwrap();

    println!("{}", lowest);
}

pub struct SeedData {
    seeds: Vec<(u64, u64)>,
    dest_source_cats: [Vec<[u64; 3]>; 7],
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

    pub fn get_lowest_location(&self, debug_print: bool) -> Result<u64, String> {
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
            println!("Starting get_lowest_seed_location...");
        }

        let mut result = u64::MAX;

        for seed_range in self.seeds.iter() {
            let curr_range_lowest = self.parse_lowest_seed_loc(*seed_range, debug_print);
            if curr_range_lowest < result {
                result = curr_range_lowest;
            }
        }

        return Ok(result);
    }

    fn parse_lowest_seed_loc(&self, seed_range: (u64, u64), debug_print: bool) -> u64 {
        let mut loc_ranges = vec![seed_range];
        for i in 0..self.dest_source_cats.len() {
            if debug_print {
                println!(
                    "Starting category {} for seed_range ({}, {})",
                    i + 1,
                    seed_range.0,
                    seed_range.1
                );
            }

            loc_ranges = loc_ranges
                .iter()
                // We get back a vector of vectors of potential destinations.
                .map(|r| self.parse_source_range_to_dest_ranges(*r, i))
                .collect::<Vec<Vec<(u64, u64)>>>()
                // We need to flatten those vectors, which will make more destination ranges to
                // work with.
                .into_iter()
                .flatten()
                .collect();

            if debug_print {
                println!(
                    "state after running category {} calculations: {:?}",
                    i + 1,
                    loc_ranges
                );
            }
        }

        let mut result = u64::MAX;
        for (loc_min, _) in loc_ranges {
            if loc_min < result {
                result = loc_min;
            }
        }
        if debug_print {
            println!(
                "lowest for seed_range ({}, {}): {}",
                seed_range.0, seed_range.1, result
            );
        }
        return result;
    }

    fn parse_source_range_to_dest_ranges(
        &self,
        source_range: (u64, u64),
        dest_source_cat_index: usize,
    ) -> Vec<(u64, u64)> {
        let mut result = vec![];
        for dest_source in self.dest_source_cats[dest_source_cat_index].iter() {
            if !Self::is_in_range(
                source_range.0,
                source_range.1,
                dest_source[1],
                dest_source[2],
            ) {
                continue;
            }

            let mut lower_cutoff: Option<(u64, u64)> = None;
            let mut upper_cutoff: Option<(u64, u64)> = None;

            let lowest_dest = match source_range.0 < dest_source[1] {
                true => {
                    let lower_cutoff_min = source_range.0;
                    let lower_cutoff_range = dest_source[1] - source_range.0;
                    if lower_cutoff_range != 0 {
                        lower_cutoff = Some((lower_cutoff_min, lower_cutoff_range));
                    }

                    dest_source[0]
                }
                false => dest_source[0] + (source_range.0 - dest_source[1]),
            };

            let leftover_source_range = source_range.1 - lower_cutoff.map_or_else(|| 0, |v| v.1);
            let dest_range = match leftover_source_range > dest_source[2] {
                true => {
                    let upper_cutoff_min = lowest_dest + dest_source[2];
                    let upper_cutoff_range = leftover_source_range - dest_source[2];
                    if upper_cutoff_range != 0 {
                        upper_cutoff = Some((upper_cutoff_min, upper_cutoff_range));
                    }

                    dest_source[2]
                }
                false => leftover_source_range,
            };

            result.push((lowest_dest, dest_range));
            if let Some(lc) = lower_cutoff {
                result.push(lc);
            }
            if let Some(uc) = upper_cutoff {
                result.push(uc);
            }
        }

        if result.len() == 0 {
            result.push(source_range);
        }

        return result;
    }

    fn is_in_range(x: u64, x_range: u64, y: u64, y_range: u64) -> bool {
        return x <= y + y_range && y <= x + x_range;
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

    fn parse_seeds(l: String) -> Result<Vec<(u64, u64)>, String> {
        if l.len() == 0 {
            return Ok(vec![]);
        }

        let parse_seed = |start_num_string: &mut String,
                          range_num_string: &mut String|
         -> Result<(u64, u64), String> {
            let start = match start_num_string.parse::<u64>() {
                Ok(n) => n,
                Err(e) => return Err(e.to_string()),
            };
            let range = match range_num_string.parse::<u64>() {
                Ok(n) => n,
                Err(e) => return Err(e.to_string()),
            };
            start_num_string.clear();
            range_num_string.clear();

            return Ok((start, range));
        };

        let mut result = vec![];
        let mut start_num_string = "".to_string();
        let mut range_num_string = "".to_string();
        let mut start_loaded = false;

        let seed_trim_index = match l.find(':') {
            Some(i) => i,
            None => return Err(format!("Line contains no colon:\n\t{}", l)),
        };
        let l = l[seed_trim_index..l.len()].trim();

        for c in l.chars() {
            match c {
                '0'..='9' => {
                    if start_loaded {
                        range_num_string.push(c);
                    } else {
                        start_num_string.push(c);
                    }
                }
                _ => {
                    if start_num_string.len() == 0 {
                        continue;
                    }
                    if !start_loaded {
                        start_loaded = true;
                        continue;
                    }
                    if range_num_string.len() == 0 {
                        continue;
                    }

                    match parse_seed(&mut start_num_string, &mut range_num_string) {
                        Ok(n) => result.push(n),
                        Err(e) => return Err(e.to_string()),
                    }
                    start_loaded = false;
                }
            }
        }

        if start_num_string.len() != 0 {
            match parse_seed(&mut start_num_string, &mut range_num_string) {
                Ok(n) => result.push(n),
                Err(e) => return Err(e.to_string()),
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

        let mut result: [u64; 3] = [0, 0, 0];
        let mut result_index = 0;
        let mut numeric_string = "".to_string();

        for c in l.trim().chars() {
            match c {
                '0'..='9' => numeric_string.push(c),
                ' ' => {
                    match numeric_string.parse::<u64>() {
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
            match numeric_string.parse::<u64>() {
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
            for seed_range in self.seeds.as_slice() {
                seed_string = format!("{} ({}, {})", seed_string, seed_range.0, seed_range.1);
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

        let test_seeds = SeedVec(vec![(79, 14), (55, 13)]);
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
    fn test_input() {
        let buf = BufReader::new(open_file(TESTFILEPATH));
        let seed_data = SeedData::parse(buf, false).unwrap();
        let lowest = seed_data.get_lowest_location(false).unwrap();

        assert_eq!(lowest, 46);
    }

    #[derive(Debug)]
    struct SeedVec(Vec<(u64, u64)>);
    impl std::cmp::Eq for SeedVec {}
    impl std::cmp::PartialEq for SeedVec {
        fn eq(&self, other: &Self) -> bool {
            for i in 0..self.0.len() {
                if i >= other.0.len() {
                    return false;
                }
                if self.0[i].0 != other.0[i].0 || self.0[i].1 != other.0[i].1 {
                    return false;
                }
            }
            return true;
        }
    }

    #[derive(Debug)]
    struct SourceDestVec(Vec<[u64; 3]>);
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
}

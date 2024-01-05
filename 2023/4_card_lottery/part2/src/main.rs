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
    let debug_print = is_test || args.contains(&"-d".to_string());
    let buf = BufReader::new(open_file(file_path));

    let mut cards = CardVec::parse(buf, debug_print);

    let result: i32 = cards.walk(debug_print);
    println!("{}", result);
}

struct CardVec(Vec<Card>);
impl CardVec {
    pub fn parse(buf: BufReader<File>, debug_print: bool) -> Self {
        let mut cards = Vec::<Card>::new();

        if debug_print {
            println!("Cards:")
        }
        for l in buf.lines() {
            match l {
                Ok(l) => {
                    if l.len() == 0 {
                        continue;
                    }

                    let card = match Card::parse(l) {
                        Ok(c) => c,
                        Err(e) => panic!("{}", e),
                    };

                    if debug_print {
                        println!("{}", card);
                    }

                    cards.push(card);
                }
                Err(e) => panic!("{}", e.to_string()),
            }
        }

        return CardVec(cards);
    }

    pub fn walk(&mut self, debug_print: bool) -> i32 {
        let mut result = 0;

        for i in 0..self.0.len() {
            result += self.0[i].card_count;
            self.add_play_if_scored(i, self.0[i].calc_score(), self.0[i].card_count, debug_print);
            if debug_print {
                println!("Round {} - {}", i, self.0[i])
            }
        }

        return result;
    }

    fn add_play_if_scored(
        &mut self,
        card_id: usize,
        card_score: i32,
        card_count: i32,
        debug_print: bool,
    ) {
        if card_score <= 0 {
            return;
        }
        for id in 1..=card_score as usize {
            let card_play = card_id + id;
            if card_play >= self.0.len() {
                break;
            }
            if debug_print {
                println!("Adding {} cards to Card ID: {}", card_count, card_play);
            }
            self.0[card_play].card_count += card_count;
        }
    }
}

struct Card {
    id: usize,
    winning_nums: Vec<u8>,
    our_nums: Vec<u8>,
    card_count: i32,
}
impl Card {
    pub fn parse(line: String) -> Result<Self, String> {
        let card_trim_index = match line.find(':') {
            Some(i) => i,
            None => return Err(format!("Line contains no colon:\n\t{}", line)),
        };
        let id = match line[0..card_trim_index]
            .replace("Card", "")
            .trim()
            .parse::<usize>()
        {
            Ok(c_id) => c_id,
            Err(e) => return Err(e.to_string()),
        };
        let pipe_index = match line.find('|') {
            Some(i) => i,
            None => return Err(format!("Line contains no pipe:\n\t{}", line)),
        };
        let (winning_nums, our_nums) = (
            Self::parse_nums(&line[card_trim_index + 1..pipe_index])?,
            Self::parse_nums(&line[pipe_index + 1..line.len()])?,
        );

        return Ok(Self {
            id,
            winning_nums,
            our_nums,
            card_count: 1,
        });
    }

    pub fn calc_score(&self) -> i32 {
        let mut winning_count: i32 = 0;
        for winning_num in self.winning_nums.iter() {
            if self.our_nums.contains(&winning_num) {
                winning_count += 1;
            }
        }

        return winning_count;
    }

    fn parse_nums(s: &str) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        let mut num_string: String = "".to_string();

        for (i, c) in s.chars().enumerate() {
            match c {
                '0'..='9' => {
                    num_string.push(c);
                    if i + 1 == s.len() {
                        match num_string.parse::<u8>() {
                            Ok(n) => result.push(n),
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                }
                _ => {
                    if num_string.len() == 0 {
                        continue;
                    }
                    match num_string.parse::<u8>() {
                        Ok(n) => result.push(n),
                        Err(e) => return Err(e.to_string()),
                    }
                    num_string.clear();
                }
            }
        }

        return Ok(result);
    }
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Card ID: {}, Card Score: {}, Card Count: {}",
            self.id,
            self.calc_score(),
            self.card_count
        )
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

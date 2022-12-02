use core::panic;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    let points: u32 = read_text("./src/input.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.trim())
        .map(|e| Game::new(e.split_once(" ").unwrap()))
        .map(|game| game.p1_points())
        .sum();
    print!("{}\n", points);
}

#[derive(PartialEq, Eq)]
enum Selection {
    Rock,
    Paper,
    Scissor,
}

struct Game {
    p1: Hand,
    p2: Hand,
}
impl Game {
    pub fn new(game_desc: (&str, &str)) -> Self {
        let (first, second) = game_desc;
        Self {
            p1: (Hand::new(second)),
            p2: (Hand::new(first)),
        }
    }

    fn evaluate_game(&self) -> u32 {
        if self.p1.meets(&self.p2) {
            return 3;
        }
        if self.p1.beats(&self.p2) {
            return 6;
        }
        return 0;
    }

    pub fn p1_points(&self) -> u32 {
        self.evaluate_game() + self.p1.points
    }
}

struct Hand {
    selection: Selection,
    points: u32,
    by: String,
}
impl Hand {
    pub fn new(str: &str) -> Self {
        if str == "A" || str == "X" {
            return Self {
                selection: Selection::Rock,
                points: 1,
                by: str.to_string(),
            };
        } else if str == "B" || str == "Y" {
            return Self {
                selection: Selection::Paper,
                points: 2,
                by: str.to_string(),
            };
        } else if str == "C" || str == "Z" {
            return Self {
                selection: Selection::Scissor,
                points: 3,
                by: str.to_string(),
            };
        } else {
            panic!("Aaaaaah val: {str}");
        }
    }

    pub fn meets(&self, other: &Hand) -> bool {
        self.selection == other.selection
    }

    pub fn beats(&self, other: &Hand) -> bool {
        self.selection == Selection::Rock && other.selection == Selection::Scissor
            || self.selection == Selection::Paper && other.selection == Selection::Rock
            || self.selection == Selection::Scissor && other.selection == Selection::Paper
    }
}

fn read_text<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut buf = String::new();
    let mut file = File::open(filename)?;
    file.read_to_string(&mut buf);
    Ok(buf)
}

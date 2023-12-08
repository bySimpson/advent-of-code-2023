use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use rayon::prelude::*;
use arrayvec::ArrayString;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}

#[derive(Parser, Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct Location {
    name: ArrayString<3>,
    left: ArrayString<3>,
    right: ArrayString<3>
}

impl Location {
    pub fn parse_from_line(line: &str) -> Self {
        let stripped = line.replace("=", "").replace("(", "").replace(")", "").replace(")", "").replace(",", "");
        let mut iter = stripped.split_whitespace();
        let name = ArrayString::<3>::from(iter.next().unwrap()).unwrap();
        let left = ArrayString::<3>::from(iter.next().unwrap()).unwrap();
        let right = ArrayString::<3>::from(iter.next().unwrap()).unwrap();

        Self {
            name,
            left,
            right
        }
    }
}

#[derive(Parser, Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum Instruction {
    Left,
    Right
}

impl Instruction {
    pub fn parse_char(character: char) -> Self {
        match character {
            'R' => {
                Self::Right
            }
            'L' => {
                Self::Left
            }
            c => {
                panic!("Received unknown input direction {}", c);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    locations: HashMap<ArrayString<3>, Location>,
    instructions: Vec<Instruction>
}

impl Map {
    pub fn new(instructions: &str) -> Self {
        let instr = instructions.chars().map(|c_char| Instruction::parse_char(c_char)).collect::<Vec<Instruction>>();
        Self {
            locations: HashMap::new(),
            instructions: instr
        }
    }

    pub fn insert_line(&mut self, line: &str) {
        let c_location = Location::parse_from_line(line);
        self.locations.insert(c_location.name, c_location);
    }

    fn get_location(&self, name: &str) -> Location {
        *self.locations.get(name).unwrap()
    }

    pub fn get_steps(&self, start: &str, end: &str) -> u32 {
        let mut running = true;
        let mut c_location = self.get_location(start);
        let dest = ArrayString::<3>::from(end).unwrap();
        let mut counter: u32 = 0;
        while running {
            let c_instruction = *self.instructions.get(counter as usize% self.instructions.len()).unwrap();

            c_location = match c_instruction {
                Instruction::Left => {
                    self.get_location(&c_location.left)
                }
                Instruction::Right => {
                    self.get_location(&c_location.right)
                }
            };

            if c_location.name == dest {
                running = false;
            }

            counter+=1;
        }
        counter
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();
    let mut iter = input.iter();
    let instructions_str = iter.next().unwrap();
    let mut map = Map::new(instructions_str);
    for c_line in iter {
        if c_line.is_empty() {
            continue
        }
        map.insert_line(c_line);
    }

    println!("Part 1:\t{:?}", map.get_steps("AAA", "ZZZ"));
    Ok(())
}

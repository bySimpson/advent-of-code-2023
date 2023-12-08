use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use rayon::prelude::*;
use arrayvec::ArrayString;
use num::integer::lcm;

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
        let stripped = line.replace(['=', '(', ')', ')', ','], "");
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
        let instr = instructions.chars().map(Instruction::parse_char).collect::<Vec<Instruction>>();
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

    pub fn get_steps(&self, start: &str, end: &str) -> u64 {
        let mut running = true;
        let mut c_location = self.get_location(start);
        let dest = ArrayString::<3>::from(end).unwrap();
        let mut counter: u64 = 0;
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

    pub fn get_positions_ending_with_x(&self, ending_with: char) -> Vec<Location> {
        self.locations.par_iter().filter(|c_location| {
            let (key, _) = *c_location;
            key.chars().last().unwrap() == ending_with
        }).map(|c_location| *c_location.1).collect::<Vec<Location>>()
    }

    pub fn part_02(&self) -> u64 {
        let current_positions = self.get_positions_ending_with_x('A');
        let ending_positions = self.get_positions_ending_with_x('Z');
        let out = current_positions.par_iter().map(|start_pos| {
            let mut running = true;
            let mut c_location = *start_pos;
            let mut counter = 0;
            let mut last_iteration = 0;
            let mut last_diff = 0;
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

                if c_location.name == start_pos.name {
                    running = false;
                }
                if ending_positions.par_iter().any(|c_ending_pos| c_ending_pos.name == c_location.name) {
                    if last_diff == counter - last_iteration {
                        running = false;
                    }
                    last_diff = counter - last_iteration;
                    last_iteration = counter;
                }

                counter+=1;
            }
            last_diff
        }).collect::<Vec<u64>>();

        
        out.into_par_iter().reduce(||1, |acc, nmbr| {
            lcm(acc, nmbr)
        })
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

    println!("Part 2:\t{:?}", map.part_02());
    Ok(())
}

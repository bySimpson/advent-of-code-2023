use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

impl Spring {
    pub fn from_char(_char: char) -> Self {
        match _char {
            '#' => {
                Self::Damaged
            }
            '.' => {
                Self::Operational
            }
            '?' => {
                Self::Unknown
            }
            c => {
                panic!("Received invalid input character {}", c)
            }
        }
    }
}

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Damaged => {
                write!(f, "#")
            }
            Self::Unknown => {
                write!(f, "?")
            }
            Self::Operational => {
                write!(f, ".")
            }
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<u32>
}

impl Record {
    pub fn from_line(line: &str) -> Self {
        let (springs_str, groups_str) = line.split_once(' ').unwrap();
        let groups = groups_str.split(',').map(|c_gr| c_gr.parse::<u32>().unwrap()).collect();
        let springs = springs_str.chars().map(Spring::from_char).collect();
        Self {
            springs,
            groups
        }
    }

    pub fn amount_of_possibilities(&self) -> u32 {
        let positions_unknown = self.springs.iter().enumerate().filter(|(_, c_spring)| {
            **c_spring == Spring::Unknown
        }).map(|(index, _)| index as u32).collect::<Vec<u32>>();
        //println!("{:?}", positions_unknown);
        let amount = positions_unknown.len();

        let all_possibilities = (0..2_u32.pow(amount as u32)).map(|i| {
            let c_possibility = (0..self.springs.len()).map(|c_position| {
                if positions_unknown.contains(&(c_position as u32)) {
                    let pos_in_unknown = positions_unknown.iter().position(|nmbr| *nmbr == c_position as u32).unwrap();
                    // replace with generated value!
                    match (i & (1<<pos_in_unknown)) != 0 {
                        true => {
                            Spring::Operational
                        }
                        false => {
                            Spring::Damaged
                        }
                    }
                }
                else {
                    //change nothing!
                    *self.springs.get(c_position).unwrap()
                }
            }).collect::<Vec<Spring>>();
            c_possibility
        }).collect::<Vec<Vec<Spring>>>();

        all_possibilities.iter().filter(|c_pos| {
            self.is_valid_possibility(c_pos)
        }).count() as u32

        //println!("{} - {:?} - {}", self, positions_unknown, amount);
    }

    pub fn is_valid_possibility(&self, input: &[Spring]) -> bool {
        let mut c_group_size = 0;
        let mut c_group = 0;
        let mut is_valid = true;
        input.iter().for_each(|c_spring| {
           match c_spring {
               Spring::Damaged => {
                   c_group_size += 1;
               }
               Spring::Operational => {
                   if c_group_size != 0 {
                       if self.groups.len() <= c_group || c_group_size != *self.groups.get(c_group).unwrap() {
                           is_valid = false;
                       }
                       c_group += 1;
                       c_group_size = 0;
                   }
               }
               Spring::Unknown => {
                   panic!("You shall not use this method with unknown values!")
               }
           }
        });
        if *input.last().unwrap() == Spring::Damaged {
            if self.groups.len() <= c_group || c_group_size != *self.groups.get(c_group).unwrap() {
                is_valid = false;
            }
            c_group += 1;
        }

        if c_group != self.groups.len() {
            is_valid = false;
        }
        //input.iter().for_each(|c_item| print!("{}", c_item));
        //println!(" - {} - {}", is_valid, c_group);
        is_valid
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.springs.iter().for_each(|c_spring| {
           write!(f, "{}", c_spring).unwrap();
        });
        write!(f, " ").unwrap();
        write!(f, "{}", self.groups.iter().map(|c_item| {
            c_item.to_string()
        }).join(","))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();

    let records = input.iter().map(|c_line| Record::from_line(c_line)).collect::<Vec<Record>>();
    let part_01 = records.par_iter().map(|c_record| {
        c_record.amount_of_possibilities()
    }).sum::<u32>();

    println!("Part 1:\t{}", part_01);
    Ok(())

}

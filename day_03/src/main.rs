use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fmt::write;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{arg, Parser};
use clap::builder::Str;
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Field {
    Number(char),
    Dot,
    Gear,
    Other(char)
}

impl Field {
    pub fn new(input: char) -> Self {
        match input.is_numeric() {
            true => {
                Field::Number(input)
            },
            false => {
                match input {
                    '.' => {
                        Field::Dot
                    }
                    '*' => {
                        Field::Gear
                    }
                    _ => {
                        Field::Other(input)
                    }
                }
            }
        }
    }
}

impl fmt::Display for Field {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            Field::Number(n) => {
                write!(f, "{}", n)
            }
            Field::Dot => {
                write!(f, ".")
            }
            Field::Gear => {
                write!(f, "*")
            }
            Field::Other(o) => {
                write!(f, "{}", o)
            }
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Engine {
    pub field: Vec<Vec<Field>>
}

impl Engine {
    pub fn new() -> Self {
        Self {
            field: vec![]
        }
    }

    pub fn insert_row(&mut self, input_row: &str) {
        let mut row: Vec<Field> = vec![];
        for c_char in input_row.chars() {
            row.push(Field::new(c_char))
        }
        let _ = &self.field.push(row);
    }

    pub fn get_position(&self, x: usize, y: usize) -> Field {
        self.field.get(y).unwrap().get(x).unwrap().to_owned()
    }

    pub fn get_valid_part_numbers(&self) -> Vec<u64> {
        let mut numbers: Vec<u64> = vec![];

        for y in 0..self.field.len() {
            let mut c_number: String = String::new();
            let mut start_coord = (0, 0);
            for x in 0..self.field.get(0).unwrap().len() {
                //let c_item = self.field.get(y)?.get(x)?;
                match self.get_position(x, y) {
                    Field::Number(n) => {
                        if c_number.is_empty() {
                            start_coord = (x, y);
                        }
                        c_number.push(n)
                    }
                    _ => {
                        if c_number.len() > 0 {
                            if self.is_valid_number(start_coord, c_number.len()) {
                                numbers.push(c_number.parse().unwrap());
                            }
                        }
                        c_number = String::new();
                    }
                }
            }
            if c_number.len() > 0 {
                if self.is_valid_number(start_coord, c_number.len()) {
                    numbers.push(c_number.parse().unwrap());
                }
            }
        }
        numbers
    }

    pub fn is_valid_number(&self, start_coord_nmbr: (usize, usize), len: usize) -> bool {
        let end_coord_nmbr = (start_coord_nmbr.0 + len -1, start_coord_nmbr.1);
        let mut start_coord_area = start_coord_nmbr;
        let mut end_coord_area = end_coord_nmbr;

        if start_coord_nmbr.0 != 0 {
            start_coord_area.0 -= 1;
        }
        if start_coord_nmbr.1 != 0 {
            start_coord_area.1 -= 1;
        }
        if end_coord_nmbr.0 != self.field.get(0).unwrap().len()-1 {
            end_coord_area.0 += 1;
        }
        if end_coord_nmbr.1 != self.field.len()-1 {
            end_coord_area.1 += 1;
        }

        for y in start_coord_area.1..=end_coord_area.1 {
            for x in start_coord_area.0..=end_coord_area.0 {
                match self.get_position(x, y) {
                    Field::Other(_) | Field::Gear => return true,
                    _ => ()
                }
            }
        }
        false
    }

    pub fn get_gear_positions(&self) -> Vec<(usize, usize)> {
        let mut gears: Vec<(usize, usize)> = vec![];

        for y in 0..self.field.len() {
            let mut c_number: String = String::new();
            let mut start_coord = (0, 0);
            for x in 0..self.field.get(0).unwrap().len() {
                //let c_item = self.field.get(y)?.get(x)?;
                match self.get_position(x, y) {
                    Field::Gear => {
                        gears.push((x, y))
                    }
                    _ => {

                    }
                }
            }
        }
        gears
    }

    pub fn get_gear_ratios(&self) -> Vec<u64> {
        let gears = self.get_gear_positions();

        let mut ratios = vec![];

        for (gear_x, gear_y) in gears {
            let mut c_starting_position = (gear_x, gear_y);
            let mut c_end_position = (gear_x, gear_y);
            let mut numbers: HashMap<(usize, usize), String> = HashMap::new();
            if gear_x != 0 {
                c_starting_position.0 -= 1;
            }
            if gear_y != 0 {
                c_starting_position.1 -= 1;
            }
            if gear_x != self.field.get(0).unwrap().len()-1 {
                c_end_position.0 += 1;
            }
            if gear_y != self.field.len()-1 {
                c_end_position.1 += 1;
            }

            for x in c_starting_position.0..=c_end_position.0 {
                for y in c_starting_position.1..=c_end_position.1 {
                   match self.get_position(x, y) {
                       Field::Number(n) => {
                           let mut traverse_x = x;
                           let mut check_next = true;
                           // find starting position of number!
                           while check_next {
                               match self.get_position(traverse_x, y) {
                                   Field::Number(_) => {
                                       if traverse_x == 0 {
                                           break;
                                       }
                                       traverse_x-=1;
                                   }
                                   _ => {
                                       check_next = false;
                                       // set ptr to last successful traverse!
                                       traverse_x += 1;
                                   }
                               }
                           }
                           check_next = true;
                           let starting_position_x = traverse_x;
                           let mut c_number: String = String::new();
                           while traverse_x != self.field.len() && check_next {
                               match self.get_position(traverse_x, y) {
                                   Field::Number(n) => {
                                       c_number.push(n);
                                       traverse_x+=1;
                                   }
                                   _ => check_next = false
                               }
                           }
                           if !c_number.is_empty() {
                               numbers.insert((starting_position_x, y), c_number);
                           }
                       }
                       _ => {

                       }
                   }
                }
            }
            if numbers.len() == 2 {
                ratios.push(numbers.into_iter().map(|c_item| c_item.1.parse::<u64>().unwrap()).product::<u64>().try_into().unwrap());
            }
        }

        ratios
    }
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c_row in &self.field {
            for c_line in c_row {
                write!(f, "{}", c_line);
            }
            write!(f, "\n");
        }
        return write!(f, "");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();
    let mut engine = Engine::new();
    let _ = input.into_iter().map(|c_line| engine.insert_row(&c_line)).collect::<Vec<_>>();

    let valid_numbers = engine.get_valid_part_numbers();
    println!("Part 1:\t{:?}", valid_numbers.iter().sum::<u64>());

    println!("{:?}", engine.get_gear_ratios().iter().sum::<u64>());
    Ok(())
}

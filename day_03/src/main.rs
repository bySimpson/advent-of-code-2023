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

    pub fn get_valid_part_numbers(&self) -> Vec<u32> {
        let mut numbers: Vec<u32> = vec![];

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
        if end_coord_nmbr.1 != self.field.get(0).unwrap().len()-1 {
            end_coord_area.1 += 1;
        }
        if end_coord_nmbr.0 != self.field.len()-1 {
            end_coord_area.0 += 1;
        }

        println!("Searching {:?} - {:?}", start_coord_area, end_coord_area);

        for y in start_coord_area.1..=end_coord_area.1 {
            for x in start_coord_area.0..=end_coord_area.0 {
                match self.get_position(x, y) {
                    Field::Other(_) => return true,
                    _ => ()
                }
            }
        }
        false
    }
}

impl fmt::Display for Engine {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
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

    println!("{}", engine);

    println!("{}", engine.field.get(0).unwrap().get(2).unwrap());
    let valid_numbers = engine.get_valid_part_numbers();
    println!("{:?}", valid_numbers.iter().sum::<u32>());

    //println!("{:?}", engine.is_valid_number((6, 2), 3));
    Ok(())
}

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use itertools::Itertools;
use rayon::prelude::*;
use crate::Direction::{South, West, East, North};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    West,
    East
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum FieldType {
    Round,
    Cube,
    Empty
}

impl FieldType {
    pub fn from_char(_char: char) -> Self {
        match _char {
            '#' => {
                Self::Cube
            }
            'O' => {
                Self::Round
            }
            '.' => {
                Self::Empty
            }
            c => {
                panic!("Received invalid input character {}", c)
            }
        }
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::Cube => {
                write!(f, "#")
            }
            FieldType::Round => {
                write!(f, "O")
            }
            FieldType::Empty => {
                write!(f, ".")
            }
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Field {
    grid: Vec<Vec<FieldType>>
}

impl Field {
    pub fn new() -> Self {
        Self {
            grid: vec![]
        }
    }

    pub fn push_line(&mut self, line: &str) {
        let c_line = line.chars().map(FieldType::from_char).collect::<Vec<FieldType>>();
        self.grid.push(c_line);
    }

    pub fn get_position(&self, x: usize, y: usize) -> FieldType {
        *self.grid.get(y).unwrap().get(x).unwrap()
    }

    pub fn get_position_mut(&mut self, x: usize, y: usize) -> &mut FieldType {
        self.grid.get_mut(y).unwrap().get_mut(x).unwrap()
    }

    pub fn get_column(&self, index: usize) -> Vec<FieldType> {
        let mut out: Vec<FieldType> = vec![];
        for y in 0..self.grid.len() {
            out.push(self.get_position(index, y));
        }
        out
    }

    pub fn simulate_part_01(&mut self) {
        loop {
            if self.simulate(North) == 0 {
                break
            }
        }
    }

    pub fn simulate_part_02(&mut self, sequence: Vec<Direction>, iterations: u32) {
        let mut seen: HashMap<Vec<Vec<FieldType>>, u32> = HashMap::new();
        for i in 1..iterations {
            for c_sequence in sequence.iter() {
                loop {
                    if self.simulate(*c_sequence) == 0 {
                        break
                    }
                }
            }
            if let Some(seen_at) = seen.insert(self.grid.clone(), i) {
                if (1000000000 - i) % (i - seen_at) == 0 {
                    break;
                }
            }
        }
    }

    pub fn simulate(&mut self, direction: Direction) -> u32 {
        let mut out = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid.first().unwrap().len() {
                let mut diff_x = x;
                let mut diff_y = y;
                match direction {
                    North => {
                        if y == 0 {
                            continue
                        }
                        diff_y -= 1;
                    }
                    South => {
                        if y == self.grid.len()-1 {
                            continue
                        }
                        diff_y += 1;
                    }
                    West => {
                        if x == 0 {
                            continue
                        }
                        diff_x -= 1;
                    }
                    East => {
                        if x == self.grid.first().unwrap().len()-1 {
                            continue
                        }
                        diff_x += 1;
                    }
                }
                let other = self.get_position(diff_x, diff_y);
                let current = self.get_position(x, y);

                if other == FieldType::Empty && current == FieldType::Round {
                    *self.get_position_mut(x, y) = FieldType::Empty;
                    *self.get_position_mut(diff_x, diff_y) = FieldType::Round;
                    out += 1;
                }
            }
        }
        out
    }

    pub fn get_points_part_01(&self) -> u64 {
        self.grid.par_iter().enumerate().map(|(index, c_line)| {
            ((self.grid.len()-index) * c_line.par_iter().filter(|c_f| **c_f == FieldType::Round).count()) as u64
        }).sum()
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for c_row in self.grid.iter() {
            for c_line in c_row {
                write!(f, "{}", c_line).unwrap();
            }
            writeln!(f).unwrap();
        }
        write!(f, "")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();

    let mut field = Field::new();

    input.iter().for_each(|c_line| field.push_line(c_line));

    let mut field_1 = field.clone();
    field_1.simulate_part_01();

    println!("Part 1:\t{}", field_1.get_points_part_01());

    field.simulate_part_02(vec![North, West, South, East], 1000000000);

    println!("Part 2:\t{}", field.get_points_part_01());
    Ok(())
}

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
enum FieldType {
    Ash,
    Rock
}

impl FieldType {
    pub fn from_char(_char: char) -> Self {
        match _char {
            '#' => {
                Self::Rock
            }
            '.' => {
                Self::Ash
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
            FieldType::Rock => {
                write!(f, "#")
            }
            FieldType::Ash => {
                write!(f, ".")
            }
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Formation {
    grid: Vec<Vec<FieldType>>
}

impl Formation {
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
        self.grid.get(y).unwrap().get(x).unwrap().to_owned()
    }

    pub fn find_row_mirror(&self, allowed_smacks: i64) -> i64 {
        for i in 0..self.grid.len() {
            let mut is_valid = false;
            let mut c_diff = 0;
            let mut smacked = false;
            loop {
                if c_diff != 0 && ((i as i64 - c_diff as i64) < 0 || (i + c_diff+1) > (self.grid.len()-1)) {
                    if smacked || allowed_smacks == 0 {
                        is_valid = true;
                    }
                    break
                }
                if (i as i64 - c_diff as i64) < 0 || (i + c_diff+1) > (self.grid.len()-1) {
                    break
                }
                let prev_row = self.grid.get(i-c_diff).unwrap();
                let fut_row = self.grid.get(i+c_diff+1).unwrap();
                if *prev_row != *fut_row {
                    if !smacked && allowed_smacks >= Formation::get_amount_differences(prev_row.clone(), fut_row.clone()) {
                        smacked = true;
                    } else {
                        break
                    }
                }
                c_diff += 1;
            }
            if is_valid {
                return i as i64 +1
            }
        }
        0
    }

    pub fn get_column(&self, index: usize) -> Vec<FieldType> {
        let mut out: Vec<FieldType> = vec![];
        for y in 0..self.grid.len() {
            out.push(self.get_position(index, y));
        }
        out
    }

    pub fn get_amount_differences(row_a: Vec<FieldType>, row_b: Vec<FieldType>) -> i64 {
        let out = row_a.iter().interleave(&row_b).tuples().filter(|(left, right)| left != right).count();
        out as i64
    }

    pub fn find_column_mirror(&self, allowed_smacks: i64) -> i64 {
        for i in 0..self.grid.first().unwrap().len() {
            let mut is_valid = false;
            let mut c_diff = 0;
            let mut smacked = false;
            loop {
                if c_diff != 0 && ((i as i64 - c_diff as i64) < 0 || (i + c_diff+1) > (self.grid.first().unwrap().len()-1)) {
                    if smacked || allowed_smacks == 0 {
                        is_valid = true;
                    }
                    break
                }
                if (i as i64 - c_diff as i64) < 0 || (i + c_diff+1) > (self.grid.first().unwrap().len()-1) {
                    break
                }
                let prev_column = self.get_column(i-c_diff);
                let fut_column = self.get_column(i+c_diff+1);
                if *prev_column != *fut_column {
                    if !smacked && allowed_smacks >= Formation::get_amount_differences(prev_column, fut_column) {
                        smacked = true
                    } else {
                        break
                    }
                }
                c_diff += 1;
            }
            if is_valid {
                return i as i64 +1
            }
        }
        0
    }

    pub fn solve(&self, allowed_smacks: i64) -> i64 {
        let out = self.find_row_mirror(allowed_smacks) * 100 + self.find_column_mirror(allowed_smacks);
        out
    }
}

impl fmt::Display for Formation {
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
    let mut formations: Vec<Formation> = vec![];
    let mut c_formation = Formation::new();
    input.iter().for_each(|c_line| {
        if c_line.is_empty() {
            formations.push(c_formation.clone());
            c_formation = Formation::new();
        } else {
            c_formation.push_line(c_line);
        }
    });
    formations.push(c_formation);

    let part_01 = formations.par_iter()
        .map(|c_formation| c_formation.solve(0))
        .sum::<i64>();

    let part_02 = formations.par_iter()
        .map(|c_formation| c_formation.solve(1))
        .sum::<i64>();

    println!("Part 1:\t{}", part_01);
    println!("Part 2:\t{}", part_02);
    Ok(())
}

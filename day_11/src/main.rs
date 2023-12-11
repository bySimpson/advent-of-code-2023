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
enum Space {
    Galaxy,
    EmptySpace,
}

impl Space {
    pub fn from_char(c_char: char) -> Self {
        match c_char {
            '.' => {
                Self::EmptySpace
            }
            '#' => {
                Self::Galaxy
            }
            c => {
                panic!("Received invalid input char {} as input!", c)
            }
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Space::Galaxy => {
                write!(f, "#")
            }
            Space::EmptySpace => {
                write!(f, ".")
            }
        }
    }
}


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Universe {
    grid: Vec<Vec<Space>>
}

impl Universe {
    pub fn new() -> Self {
        Self {
            grid: vec![]
        }
    }

    pub fn get_position(&self, x: usize, y: usize) -> Space {
        self.grid.get(y).unwrap().get(x).unwrap().to_owned()
    }

    pub fn insert_row(&mut self, line: &str) {
        let mut c_row: Vec<Space> = vec![];
        line.chars().for_each(|c_char| {
           c_row.push(Space::from_char(c_char));
        });

        if c_row.par_iter().all(|c_space| *c_space == Space::EmptySpace) {
            self.grid.push(c_row.clone());
        }
        self.grid.push(c_row);
    }

    pub fn manhattan(&self, point_a: (isize, isize), point_b: (isize, isize)) -> i64 {
        let diff_x = point_a.0 - point_b.0;
        let diff_y = point_a.1 - point_b.1;
        (diff_x.abs() + diff_y.abs()) as i64

    }

    pub fn get_coords_of_all_galaxies(&self) -> Vec<(isize, isize)> {
        let mut out: Vec<(isize, isize)> = vec![];
        for c_y in 0..self.grid.len() {
            for c_x in 0..self.grid.first().unwrap().len() {
                let c_field = self.get_position(c_x, c_y);
                if let Space::Galaxy = c_field {
                    out.push((c_x as isize, c_y as isize));
                }
            }
        }
        out
    }

    pub fn create_space(&mut self) {
        let mut out: Vec<Vec<Space>> = vec![];
        for c_y in 0..self.grid.len() {
            let mut c_line: Vec<Space> = vec![];
            for c_x in 0..self.grid.first().unwrap().len() {
                let mut is_empty = true;
                for c_inner_y in 0..self.grid.len() {
                    let c_line_item = self.get_position(c_x, c_inner_y);

                    if let Space::Galaxy = c_line_item {
                        is_empty = false;
                        break;
                    }
                }
                let c_item = self.get_position(c_x, c_y);
                c_line.push(c_item);
                if is_empty {
                    c_line.push(c_item);
                }
            }
            out.push(c_line);
        }

        self.grid = out;
    }

    pub fn part_01(&self) -> i64 {
        let coords_galaxies = self.get_coords_of_all_galaxies();
        println!("{:?}", coords_galaxies);

        /*coords_galaxies.iter().map(|c_coords| {
            coords_galaxies.iter().fold(0, |mut acc, c_inner_coords| {
                let manhattan = self.manhattan(*c_coords, *c_inner_coords);
                println!("{:?} - {:?}: {}", c_coords, c_inner_coords, manhattan);
                acc += manhattan;
                acc
            })
        }).sum::<i64>();*/

        coords_galaxies.iter().tuple_combinations().map(|(left, right)| {
            //println!("{:?}, {:?} - {}", left, right, self.manhattan(*left, *right));
            self.manhattan(*left, *right)
        }).sum::<i64>()
    }
}

impl fmt::Display for Universe {
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

    let mut universe = Universe::new();
    input.iter().for_each(|c_row| {
       universe.insert_row(c_row);
    });

    universe.create_space();
    //println!("{}", universe);

    println!("{}", universe.part_01());
    Ok(())
}

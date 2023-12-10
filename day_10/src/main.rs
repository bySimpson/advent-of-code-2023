use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use rayon::prelude::*;
use crate::Direction::{Down, Left, Right, Up};

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
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Pipe {
    Start,
    Horizontal,
    Vertical,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    Ground
}

impl Pipe {
    pub fn new(character: char) -> Self {
        match character {
            '|' => {
                Self::Vertical
            }
            '-' => {
                Self::Horizontal
            }
            'L' => {
                Self::UpRight
            }
            'J' => {
                Self::UpLeft
            }
            '7' => {
                Self::DownLeft
            }
            'F' => {
                Self::DownRight
            }
            '.' => {
                Self::Ground
            }
            'S' => {
                Self::Start
            }
            c => {
                panic!("Received unknown character '{}' as pipe input", c)
            }
        }
    }

    pub fn get_directions(&self) -> Vec<Direction> {
        match self {
            Pipe::Start => {
                vec![Up, Down, Left, Right]
            }
            Pipe::Horizontal => {
                vec![Left, Right]
            }
            Pipe::Vertical => {
                vec![Up, Down]
            }
            Pipe::UpRight => {
                vec![Up, Right]
            }
            Pipe::UpLeft => {
                vec![Up, Left]
            }
            Pipe::DownLeft => {
                vec![Down, Left]
            }
            Pipe::DownRight => {
                vec![Down, Right]
            }
            Pipe::Ground => {
                vec![]
            }
        }
    }

    pub fn get_next_direction(&self, coming_from: Direction) -> Option<Direction> {
        let out = self.get_directions().iter().filter(|c_direction| **c_direction != coming_from).copied().collect::<Vec<Direction>>();
        match out.len() {
            1 => {
                Some(*out.first().unwrap())
            }
            _ => {
                println!("{:?}", out);
                None
            }
        }
    }
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pipe::Start => {
                write!(f, "S")
            }
            Pipe::Horizontal => {
                write!(f, "═")
            }
            Pipe::Vertical => {
                write!(f, "║")
            }
            Pipe::UpRight => {
                write!(f, "╚")
            }
            Pipe::UpLeft => {
                write!(f, "╝")
            }
            Pipe::DownLeft => {
                write!(f, "╗")
            }
            Pipe::DownRight => {
                write!(f, "╔")
            }
            Pipe::Ground => {
                write!(f, ".")
            }
        }
    }
}

struct Area {
    pipes: Vec<Vec<Pipe>>,
    starting_position: (u64, u64)
}

impl Area {
    pub fn new() -> Self {
        Self {
            pipes: vec![],
            starting_position: (0, 0)
        }
    }

    pub fn get_position(&self, x: usize, y: usize) -> Pipe {
        self.pipes.get(y).unwrap().get(x).unwrap().to_owned()
    }

    pub fn instert_row(&mut self, row: &str) {
        let mut c_x_pos = 0;
        let c_vec = row.chars().map(|c_char| {
            let pipe = Pipe::new(c_char);
            if pipe == Pipe::Start {
                self.starting_position = (c_x_pos, self.pipes.len() as u64);
            }
            c_x_pos += 1;
            pipe
        }).collect::<Vec<Pipe>>();
        self.pipes.push(c_vec);

    }

    pub fn part_01(&mut self) -> u64 {
        let mut c_position = self.starting_position;
        let mut c_pipe = self.get_position(c_position.0 as usize, c_position.1 as usize);
        let mut c_direction = if c_position.0 != 0 && self.get_position(c_position.0 as usize - 1, c_position.1 as usize).get_directions().contains(&Direction::Right) {
            Direction::Left
        } else if c_position.0 != self.pipes.len() as u64 - 1 && self.get_position(c_position.0 as usize + 1, c_position.1 as usize).get_directions().contains(&Direction::Left) {
            Direction::Right
        } else if c_position.1 != 0 && self.get_position(c_position.0 as usize, c_position.1 as usize - 1).get_directions().contains(&Direction::Down) {
            Direction::Up
        } else {
            println!("Warning: Default value DOWN selected!");
            Direction::Down
        };
        //let mut c_direction = Direction::Down; // TODO: Logic for start!
        let mut counter = 0;
        loop {
            if c_pipe == Pipe::Start && counter != 0 {
                break
            }

            match c_direction {
                Left => {
                    c_position.0 -= 1;
                }
                Right => {
                    c_position.0 += 1;
                }
                Up => {
                    c_position.1 -= 1;
                }
                Down => {
                    c_position.1 += 1;
                }
            };

            c_pipe = self.get_position(c_position.0 as usize, c_position.1 as usize);

            // we have to flip the current position!
            c_direction = match c_direction {
                Left => {
                    Right
                }
                Right => {
                    Left
                }
                Up => {
                    Down
                }
                Down => {
                    Up
                }
            };

            if c_pipe != Pipe::Start && c_pipe != Pipe::Ground {
                c_direction = c_pipe.get_next_direction(c_direction).unwrap();
            }

            counter += 1;
        }
        counter/2
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c_row in &self.pipes {
            for c_line in c_row {
                write!(f, "{}", c_line).unwrap();
            }
            writeln!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();
    let mut area = Area::new();
    input.iter().for_each(|c_row| {
        area.instert_row(c_row);
    });

    //println!("{}", area);

    println!("Part 1:\t{:?}", area.part_01());
    Ok(())
}
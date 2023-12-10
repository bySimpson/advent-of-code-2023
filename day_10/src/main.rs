use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};
use clap::Parser;
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

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Encloser {
    Loop,
    UnInclosed,
    Unprocessed
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

#[derive(Debug, Clone)]
struct Ring(Vec<Vec<Encloser>>);

struct Area {
    pipes: Vec<Vec<Pipe>>,
    starting_position: (u64, u64),
    ring_part_2: Ring,
}

impl Area {
    pub fn new() -> Self {
        Self {
            pipes: vec![],
            ring_part_2: Ring(vec![]),
            starting_position: (0, 0)
        }
    }

    pub fn get_position(&self, x: usize, y: usize) -> Pipe {
        self.pipes.get(y).unwrap().get(x).unwrap().to_owned()
    }

    pub fn get_position_ring(&self, x: usize, y: usize) -> Encloser {
        self.ring_part_2.get(y).unwrap().get(x).unwrap().to_owned()
    }

    pub fn get_position_ring_mut(&mut self, x: usize, y: usize) -> &mut Encloser {
        self.ring_part_2.get_mut(y).unwrap().get_mut(x).unwrap()
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
        let mut c_direction = Direction::Up;
        if c_position.0 != 0 && self.get_position(c_position.0 as usize - 1, c_position.1 as usize).get_directions().contains(&Direction::Right) {
            *self.get_position_ring_mut(c_position.0 as usize *2 +1, c_position.1 as usize *2) = Encloser::Loop;
            c_direction = Direction::Left
        }
        if c_position.0 != self.pipes.len() as u64 - 1 && self.get_position(c_position.0 as usize + 1, c_position.1 as usize).get_directions().contains(&Direction::Left) {
            c_direction = Direction::Right
        }
        if c_position.1 != 0 && self.get_position(c_position.0 as usize, c_position.1 as usize - 1).get_directions().contains(&Direction::Down) {
            *self.get_position_ring_mut(c_position.0 as usize *2, c_position.1 as usize *2 +1) = Encloser::Loop;
            c_direction = Direction::Up
        }
        if c_position.0 != self.pipes.first().unwrap().len() as u64 - 1 && self.get_position(c_position.0 as usize, c_position.1 as usize + 1).get_directions().contains(&Direction::Up) {
            c_direction = Direction::Down
        };
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

            *self.get_position_ring_mut(c_position.0 as usize *2, c_position.1 as usize *2) = Encloser::Loop;
            if c_pipe.get_directions().contains(&Direction::Down) && c_pipe != Pipe::Start {
                *self.get_position_ring_mut(c_position.0 as usize *2, c_position.1 as usize *2 +1) = Encloser::Loop;
            }

            if c_pipe.get_directions().contains(&Direction::Right) && c_pipe != Pipe::Start {
                *self.get_position_ring_mut(c_position.0 as usize *2 +1, c_position.1 as usize *2) = Encloser::Loop;
            }

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

    pub fn init_part_02(&mut self) {
        self.ring_part_2 = Ring(vec![vec![Encloser::Unprocessed; self.pipes.get(0).unwrap().len()*2];self.pipes.len()*2]);
    }

    pub fn part_02(&mut self) -> u64 {
        loop {
            let mut processed_this_iteration = 0;
            for c_y in 0..self.ring_part_2.len() {
                if self.get_position_ring(0, c_y) != Encloser::Loop {
                    *self.get_position_ring_mut(0, c_y) = Encloser::UnInclosed;
                }
                if self.get_position_ring(self.ring_part_2.first().unwrap().len() -1, c_y) != Encloser::Loop {
                    *self.get_position_ring_mut(self.ring_part_2.first().unwrap().len() -1, c_y) = Encloser::UnInclosed;
                }
                for c_x in 0..self.ring_part_2.first().unwrap().len() {
                    if c_y == 0 {
                        if self.get_position_ring(c_x, 0) != Encloser::Loop {
                            *self.get_position_ring_mut(c_x, 0) = Encloser::UnInclosed;
                        }
                        if self.get_position_ring(c_x, self.ring_part_2.len() -1) != Encloser::Loop{
                            *self.get_position_ring_mut(c_x, self.ring_part_2.len() -1) = Encloser::UnInclosed;
                        }
                    }
                    if ((c_x != 0 && self.get_position_ring(c_x - 1, c_y) == Encloser::UnInclosed) || // left
                        (c_y != 0 && self.get_position_ring(c_x, c_y - 1) == Encloser::UnInclosed) || // up
                        (c_x != self.ring_part_2.first().unwrap().len() -1 && self.get_position_ring(c_x + 1, c_y) == Encloser::UnInclosed) || // right
                        (c_y != self.ring_part_2.len() -1 && self.get_position_ring(c_x, c_y + 1) == Encloser::UnInclosed)) && // up
                        self.get_position_ring(c_x, c_y) == Encloser::Unprocessed {
                        processed_this_iteration += 1;
                        *self.get_position_ring_mut(c_x, c_y) = Encloser::UnInclosed;
                    }
                }
            }

            if processed_this_iteration == 0 {
                break;
            }
        }
        let mut c_y = 0;
        self.ring_part_2.iter().fold(0_u64, |mut acc, c_row| {
            let mut c_x = 0;
            acc += c_row.iter().filter(|c_item| {
                let out = **c_item == Encloser::Unprocessed && c_x % 2 == 0 && c_y % 2 == 0;
                c_x +=1;
                out
            }).count() as u64;
            c_y += 1;
            acc
        })
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c_row in &self.pipes {
            for c_line in c_row {
                write!(f, "{}", c_line).unwrap();
            }
            writeln!(f).unwrap();
        }
        write!(f, "")
    }
}

impl Deref for Ring {
    type Target = Vec<Vec<Encloser>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ring {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Ring {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for c_row in self.0.iter() {
            for c_line in c_row {
                match *c_line {
                    Encloser::Loop => {
                        write!(f, "█").unwrap();
                    }
                    Encloser::Unprocessed => {
                        write!(f, "░").unwrap();
                    }
                    Encloser::UnInclosed => {
                        write!(f, "▓").unwrap();
                    }
                }
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
    let mut area = Area::new();
    input.iter().for_each(|c_row| {
        area.instert_row(c_row);
    });

    area.init_part_02();

    println!("Part 1:\t{}", area.part_01());

    println!("Part 2:\t{}", area.part_02() -2); // Why -2?
    Ok(())
}
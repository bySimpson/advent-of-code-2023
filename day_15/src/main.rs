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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Sequence {
    characters: Vec<char>
}

impl Sequence {
    pub fn from_line(line: &str) -> Self {
        let out: Vec<char> = line.chars().collect::<Vec<char>>();
        Self {
            characters: out
        }
    }

    pub fn part_01(&self) -> u32 {
        self.characters.iter().fold(0, |mut acc, c_char| {
            acc += *c_char as u32;
            acc *= 17;
            acc %= 256;
            acc
        })
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();

    let sequences = input.first().unwrap().split(',').par_bridge().map(Sequence::from_line).collect::<Vec<Sequence>>();

    println!("Part 1:\t{}", sequences.par_iter().map(Sequence::part_01).sum::<u32>());
    Ok(())
}

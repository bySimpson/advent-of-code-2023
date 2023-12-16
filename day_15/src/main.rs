use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
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

fn hash_2(line: &str) -> usize {
    line.chars().fold(0, |mut acc, c_char| {
        acc += c_char as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();

    let mut boxes = vec![LinkedHashMap::<&str, u8>::new(); 256];
    let sequences = input.first().unwrap().split(',').par_bridge().map(Sequence::from_line).collect::<Vec<Sequence>>();

    println!("Part 1:\t{}", sequences.par_iter().map(Sequence::part_01).sum::<u32>());

    // Part02
    input.first().unwrap().split(',').for_each(|c_item| {
        let mut step = c_item.split(|c| c == '=' || c == '-');

        let label = step.next().unwrap();
        let focal_length = step.next().unwrap();

        let id = hash_2(label);

        if focal_length.is_empty() {
            boxes[id].remove(label);
        } else {
            *boxes[id].entry(label).or_insert(0) = focal_length.parse().unwrap();
        }
    });

    let part_02 = boxes.iter().enumerate().fold(0, |mut acc, (b, c_box)| {
        for i in 0..c_box.len() {
            let label = c_box.keys().nth(i).unwrap();
            acc += (b + 1) * (i + 1) * boxes[b][label] as usize;
        }
        acc
    });

    println!("Part 2:\t{}", part_02);

    Ok(())
}

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Race {
    time: u32,
    distance_to_beat: u32
}

impl Race {
    pub fn new(time: &str, distance_to_beat: &str) -> Self {
        Self {
            time: time.parse::<u32>().unwrap(),
            distance_to_beat: distance_to_beat.parse::<u32>().unwrap()
        }
    }

    pub fn calculate_distance(&self, hold_time: u32) -> u32 {
        (self.time - hold_time) * hold_time
    }

    pub fn amount_of_times_to_beat(&self) -> u32 {
        (0..=self.time).par_bridge().map(|c_hold_time| {
            self.calculate_distance(c_hold_time)
        }).filter(|achieved_distance| *achieved_distance > self.distance_to_beat).count() as u32
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();
    let mut iter = input.iter();
    let mut time_iter = iter.next().unwrap().split_whitespace().skip(1);
    let mut distance_iter = iter.next().unwrap().split_whitespace().skip(1);
    let mut races: Vec<Race> = vec![];
    for _ in 0..time_iter.clone().count() {
        races.push(Race::new(time_iter.next().unwrap(), distance_iter.next().unwrap()));
    }

    let part_1 = races.par_iter().map(|c_race| {
        c_race.amount_of_times_to_beat()
    }).product::<u32>();

    println!("Part 1:\t{:?}", part_1);
    Ok(())
}

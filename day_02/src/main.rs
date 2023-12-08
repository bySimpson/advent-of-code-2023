use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{arg, Parser};
use anyhow::{Result};
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}

#[derive(Debug, Copy, Clone)]
pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    pub fn new(round_input: String) -> Self {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for c_items in round_input.split(", ") {
            let (amount_str, cube_type_str) = c_items.split_once(' ').unwrap();
            let amount: u32 = amount_str.parse().unwrap();
            match cube_type_str {
                "red" => red = amount,
                "green" => green = amount,
                "blue" => blue = amount,
                _ => panic!("Invalid cube color inserted!")
            }
        }
        Self {
            red,
            green,
            blue,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    id: u32,
    round: Vec<Round>
}

impl Game {
    pub fn new(id: u32, round: Vec<Round>) -> Self {
        Self {
            id,
            round
        }
    }

    pub fn part_1_would_be_possible(&self, amount_red: u32, amount_green: u32, amount_blue: u32) -> bool {
        let out = &self.round.par_iter().map(|c_round|{
            if c_round.red > amount_red || c_round.green > amount_green || c_round.blue > amount_blue {
                return true;
            }
            false
        }).any(|item| item);
        !out
    }

    pub fn part_2_smallest_cube_size(&self) -> u32 {
        let mut max_red: u32 = 0;
        let mut max_green: u32= 0;
        let mut max_blue: u32 = 0;
        for c_round in &self.round {
            max_red = max_red.max(c_round.red);
            max_green = max_green.max(c_round.green);
            max_blue = max_blue.max(c_round.blue);
        }

        max_red * max_green * max_blue
    }
}


fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().par_bridge().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();
    let games = input.par_iter().map(|c_line| {
        let (game_id_str, c_game_str) = c_line.split_once(": ").unwrap();
        let game_id: u32 = game_id_str.split_once(' ').unwrap().1.parse().unwrap();
        let mut rounds: Vec<Round> = vec![];
        for c_round in c_game_str.split("; ") {
            rounds.push(Round::new(c_round.to_string()));
        }
        Game::new(game_id, rounds)
    }).collect::<Vec<Game>>();

    let part_1: u32 = games.clone().par_iter()
        .filter(|c_game| c_game.part_1_would_be_possible(12, 13, 14))
        .fold(|| 0u32, |mut acc, game| {
        acc += game.id;
        acc
    }).sum();
    let part_2: u32 = games.par_iter().map(|c_game| c_game.part_2_smallest_cube_size()).sum();

    println!("Part 1:\t{}", part_1);
    println!("Part 2:\t{}", part_2);
    Ok(())
}
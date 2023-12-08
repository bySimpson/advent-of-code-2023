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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Scratchcard {
    id: u32,
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>
}

impl Scratchcard {
    pub fn new(id: u32, winning_numbers: Vec<u32>, owned_numbers: Vec<u32>) -> Self {
        Self {
            id,
            winning_numbers,
            owned_numbers,
        }
    }

    pub fn find_winning_numbers(&self) -> Vec<u32> {
        self.winning_numbers.par_iter().filter(|c_number| {
            self.owned_numbers.contains(*c_number)
        }).map(|c_item| {
            *c_item
        }).collect::<Vec<u32>>()
    }

    pub fn calculate_points(&self) -> u32 {
        let count = self.find_winning_numbers().par_iter().count() as u32;
        if count == 0 {
            0
        }
        else {
            2_u32.pow(count - 1)
        }
    }
}

fn parse_numbers(input: &str) -> Vec<u32> {
    input.split_whitespace().par_bridge().map(|c_nmr_str| {
        c_nmr_str.parse::<u32>().unwrap()
    }).collect::<Vec<u32>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();
    // Cannot use rayon because order is important!
    let game: Vec<Scratchcard> = input.iter().map(|c_line| {
        let (card_id_str, game_input) = c_line.split_once(':').unwrap();
        let card_id = card_id_str.split_whitespace().last().unwrap().parse::<u32>().unwrap();
        let mut number_iterator = game_input.split('|');
        let winning_numbers = parse_numbers(number_iterator.next().unwrap());
        let owned_numbers = parse_numbers(number_iterator.next().unwrap());
        Scratchcard::new(card_id, winning_numbers, owned_numbers)
    }).collect();

    let part_1: u32 = game.par_iter().map(|c_scratchcard| {
        c_scratchcard.calculate_points()
    }).sum::<u32>();

    // hold amount of cards; SHIFTED BY -1!
    let mut amount_of_cards_lookup: Vec<u32> = vec![1; input.len()];

    println!("Part 1:\t{:#?}", part_1);

    // Cannot use rayon because order is important!
    game.iter().for_each(|c_scratchcard| {
        let c_points = c_scratchcard.find_winning_numbers().len();
        //let c_amount_lookup_id = c_scratchcard.id -1;
        let c_amount_lookup = *amount_of_cards_lookup.get(c_scratchcard.id as usize - 1).unwrap();
        let _ = &amount_of_cards_lookup[c_scratchcard.id as usize..c_scratchcard.id as usize+c_points].par_iter_mut().for_each(|c_number| {
            *c_number += c_amount_lookup;
        });
    });
    println!("Part 2:\t{:?}", amount_of_cards_lookup.par_iter().sum::<u32>());
    Ok(())
}

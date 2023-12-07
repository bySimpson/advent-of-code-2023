use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash, Eq, Ord)]
#[repr(u64)]
enum Card {
    Number(u64),
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn parse(input: char) -> Self {
        match input.is_numeric() {
            true => {
                let parsed: u64 = input.to_digit(10).unwrap() as u64;
                if parsed < 10 && parsed != 1 {
                    Card::Number(parsed)
                }
                else {
                    panic!("Received a Number <10!")
                }
            }
            false => {
                match input {
                    'T' => {
                      Card::Ten
                    }
                    'J' => {
                        Card::Jack
                    }
                    'Q' => {
                        Card::Queen
                    }
                    'K' => {
                        Card::King
                    }
                    'A' => {
                        Card::Ace
                    }
                    c => {
                        panic!("Invalid character {} received.", c)
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
#[repr(u64)]
pub enum Value {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FifeOfAKind,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    card_count: HashMap<Card, u64>,
    bid: u64,
    highest_value: Value
}

impl Hand {
    pub fn parse_input_line(line: &str) -> Self {
        let mut card_count = HashMap::from([
            (Card::Number(0), 0),
            (Card::Number(1), 0),
            (Card::Number(2), 0),
            (Card::Number(3), 0),
            (Card::Number(4), 0),
            (Card::Number(5), 0),
            (Card::Number(6), 0),
            (Card::Number(7), 0),
            (Card::Number(8), 0),
            (Card::Number(9), 0),
            (Card::Ten, 0),
            (Card::Jack, 0),
            (Card::Queen, 0),
            (Card::King, 0),
            (Card::Ace, 0),
        ]);

        let mut whitespace_iter = line.split_whitespace();
        let card_str = whitespace_iter.next().unwrap();
        let bid = whitespace_iter.next().unwrap().parse::<u64>().unwrap();

        let cards = card_str.chars().map(|c_char| {
            let out = Card::parse(c_char);
            *card_count.get_mut(&out).unwrap() += 1;
            out
        }).collect::<Vec<Card>>();
        let mut out = Self {
            cards,
            bid,
            card_count,
            highest_value: Value::HighCard
        };
        out.highest_value = out.get_best_hand();
        out
    }

    pub fn get_best_hand(&self) -> Value {
        let max_amount_of_cards = self.card_count.values().max().unwrap();
        match max_amount_of_cards {
            5 => {
                Value::FifeOfAKind
            }
            4 => {
                Value::FourOfAKind
            }
            3 => {
                // Full House?!

                if self.card_count.values().par_bridge().any(|c_count| *c_count == 2) {
                    // Both Three of a Kind and Two of a Kind, has to be Full House
                    return Value::FullHouse;
                }
                Value::ThreeOfAKind
            }
            2 => {
                // Two Pairs?!
                if self.card_count.values().par_bridge().filter(|c_count| **c_count == 2).count() == 2 {
                    // Two pairs
                    return Value::TwoPair
                }
                Value::OnePair
            }
            1 => {
                Value::HighCard
            }
            n => {
                panic!("Undefined amount od matches, not possible: {}", n)
            }
        }

    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {

}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.cards == other.cards {
            return Some(Equal);
        }
        else {
            if self.highest_value != other.highest_value {
                if self.highest_value < other.highest_value {
                    return Some(Less);
                }
                return Some(Greater);
            }
            else {
                let mut self_iterator = self.cards.iter();
                let mut other_iterator = other.cards.iter();
                let mut counter = 0;
                while counter < 5 {
                    let self_val = self_iterator.next().unwrap();
                    let other_val = other_iterator.next().unwrap();
                    if self_val != other_val {
                        if self_val < other_val {
                            return Some(Less);
                        }
                        return Some(Greater);
                    }
                    counter += 1;
                }
                return None;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();
    let mut game = input.iter().map(|c_line| Hand::parse_input_line(c_line)).collect::<Vec<Hand>>();

    game.sort();

    let mut counter = 1;

    let part_01 = game.iter().map(|c_hand| {
        let out = counter * c_hand.bid;
        counter += 1;
        out
    }).sum::<u64>();

    println!("Part 1:\t{}", part_01);
    Ok(())
}

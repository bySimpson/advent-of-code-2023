use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{arg, Parser};
use anyhow::{Result};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}


fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();
    // day_ß1
    println!("Part 1:\t{}", do_calculation(&input));
    // day_02
    let string_number_mapping: HashMap<String, char> = HashMap::from([
        ("one".to_string(),   '1'),
        ("two".to_string(),   '2'),
        ("three".to_string(), '3'),
        ("four".to_string(),  '4'),
        ("five".to_string(),  '5'),
        ("six".to_string(),   '6'),
        ("seven".to_string(), '7'),
        ("eight".to_string(), '8'),
        ("nine".to_string(),  '9')
    ]);

    let mut input_part_2: Vec<String> = vec![];

    for mut c_line in input.clone() {
        let mut first_number: Option<(usize, char)> = None;
        let mut last_number: Option<(usize, char)> = None;
        for (k, v) in &string_number_mapping {
            if c_line.contains(k) {
                let first_position = c_line.find(k).unwrap();
                let last_position = c_line.rfind(k).unwrap();
                if first_number.is_some() {
                    if &first_number.unwrap().0 > &first_position {
                        first_number = Some((first_position, *v))
                    }
                }
                else {
                    first_number = Some((first_position, *v))
                }

                if last_number.is_some() {
                    if &last_number.unwrap().0 < &last_position {
                        last_number = Some((last_position, *v))
                    }
                }
                else {
                    last_number = Some((last_position, *v))
                }
            }
        }
        if first_number.is_some() {
            let (c_pos, c_char) = first_number.unwrap();
            c_line.insert(c_pos, c_char);
        }
        if last_number.is_some() && first_number.is_some() && &first_number.unwrap() != &last_number.unwrap() {
            // +1 needed because everything will be shifted because of first insert
            let (c_pos, c_char) = last_number.unwrap();
            c_line.insert(c_pos+1, c_char);
        }
        input_part_2.push(c_line);
    }
    println!("Part 2:\t{}", do_calculation(&input_part_2));
    Ok(())
}

fn do_calculation(input: &Vec<String>) -> i32 {
    // Find first and last number
    let mut parsed: Vec<i32> = vec![];
    for line in input {
        let mut c_number: String = line.chars().filter(|c_char| c_char.is_numeric()).collect();
        if !c_number.is_empty() {
            c_number = c_number.chars().nth(0).unwrap().to_string() + &c_number.chars().nth_back(0).unwrap().to_string();
        }
        parsed.push(c_number.parse::<i32>().unwrap_or(0));
    };
    parsed.iter().sum::<i32>()
}

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

fn is_leading_zeroes(in_vec: &[i64]) -> bool {
    in_vec.par_iter().all(|c_number| *c_number == 0)
}

fn get_differences(in_vec: &[i64]) -> Vec<i64> {
    in_vec.windows(2).map(|c_vals| {
        c_vals[1] - c_vals[0]
    }).collect()
}

fn extrapolate(in_vec: &[i64], reversed: bool) -> i64 {
    if is_leading_zeroes(in_vec) {
        return 0
    }

    let diff_to_val = extrapolate(&get_differences(in_vec), reversed);
    if reversed {
        in_vec.first().unwrap() - diff_to_val
    } else {
        in_vec.last().unwrap() + diff_to_val
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();

    let values: Vec<Vec<i64>> = input.iter().map(|c_line| {
        let mut c_vec: Vec<i64> = vec![];
        c_line.split_whitespace().for_each(|c_item| {
            c_vec.push(c_item.parse::<i64>().unwrap());
        });
        c_vec
    }).collect();

    let parts = values.iter().fold((0, 0), |mut acc, c_values| {
        acc.0 += extrapolate(c_values, false);
        acc.1 += extrapolate(c_values, true);
        acc
    });

    println!("Part 1:\t{:?}", parts.0);
    println!("Part 2:\t{:?}", parts.1);
    Ok(())
}

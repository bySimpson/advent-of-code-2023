
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Range, RangeInclusive};
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

#[derive(Debug, Copy, Clone)]
struct GenericMapLine {
    destination: u64,
    source: u64,
    range: u64
}

impl GenericMapLine {
    pub fn new(destination: u64, source: u64, range:u64) -> Self {
        Self {
            destination,
            source,
            range
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct GenericMap {
    map_type: GenericMapType,
    pub lines: Vec<GenericMapLine>
}

impl GenericMap {
    pub fn new(map_type: GenericMapType) -> Self {
        Self {
            map_type,
            lines: vec![]
        }
    }

    pub fn insert_line(&mut self, line: &str) {
        let mut iterator = line.split_whitespace();
        let generic_map_line = GenericMapLine::new(iterator.next().unwrap().parse().unwrap(),
                                                   iterator.next().unwrap().parse().unwrap(),
                                                   iterator.next().unwrap().parse().unwrap());
        self.lines.push(generic_map_line);
    }
}

#[derive(Debug, Clone)]
struct Game {
    seeds: Vec<u64>,
    seeds_part_2: Vec<RangeInclusive<u64>>,
    maps: Vec<GenericMap>
}

impl Game {
    pub fn new() -> Self {
        Self {
            seeds: vec![],
            maps: vec![],
            seeds_part_2: vec![]
        }
    }

    pub fn set_seeds(&mut self, line: &str) {
        let iterator = line.split_whitespace();
        self.seeds = iterator.skip(1).map(|c_line| c_line.parse::<u64>().unwrap()).collect()
    }

    pub fn set_seeds_part_2(&mut self, line: &str) {
        let mut iterator = line.split_whitespace().skip(1);
        let amount_to_check = iterator.clone().count() / 2;
        let mut c_counter = 0;

        while c_counter < amount_to_check  {
            let start_val = iterator.next().unwrap().parse::<u64>().unwrap();
            let end_val = start_val + iterator.next().unwrap().parse::<u64>().unwrap();
            self.seeds_part_2.push(start_val..=end_val);
            c_counter += 1;
        }
    }

    pub fn insert_map(&mut self, genreric_map: GenericMap) {
        self.maps.push(genreric_map)
    }

    pub fn find_destination(&self, map_type: i64, source: u64) -> u64 {
        assert!(self.maps.len() >= map_type as usize);
        let c_map = self.maps.get(map_type as usize).unwrap();
        let found_map_line = c_map.lines.par_iter().find_any(|generic_map_line: &&GenericMapLine| {
           generic_map_line.source <= source && source <= generic_map_line.source + generic_map_line.range -1 // TODO: -1=!
        });

        match found_map_line {
            Some(gml) => {
                let out = (gml.destination as i64 + (source as i64 - gml.source as i64)) as u64;
                return out;
            }
            None => {
                // same source
                return source;
            }
        }
    }

    pub fn traverse(&self, c_seed: u64) -> u64 {
        let mut c_location_map = 0;
        let mut c_nmbr = c_seed;
        while c_location_map <= GenericMapType::HumidityToLocation as i64 {
            c_nmbr = self.find_destination(c_location_map, c_nmbr);
            c_location_map += 1;
        }
        c_nmbr
    }

    pub fn play_part_1(&self) -> u64 {
        assert_ne!(self.seeds.len(), 0);
        self.seeds.par_iter().map(|c_seed| {
            self.traverse(*c_seed)
        }).min().unwrap()
    }

    pub fn play_part_2(&self) -> u64 {

        self.seeds_part_2.clone().into_par_iter().map(|c_seed_range| {
            c_seed_range.par_bridge().map(|c_seed| {
                self.traverse(c_seed)
            }).min().unwrap()
        }).min().unwrap()
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum GenericMapType {
    SeedToSoil = 0,
    SoilToFertilizer = 1,
    FertilizerToWater = 2,
    WaterToLight = 3,
    LightToTemperature = 4,
    TemperatureToHumidity = 5,
    HumidityToLocation = 6
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();
    let mut empty_lines_counter = -1;
    let mut game = Game::new();
    let mut c_generic_map = GenericMap::new(GenericMapType::SeedToSoil);
    for c_line in input {
        if c_line.is_empty() {
            empty_lines_counter += 1;
            if empty_lines_counter > 0 {
                let generic_map_type: GenericMapType = unsafe { ::std::mem::transmute(empty_lines_counter) };
                game.insert_map(c_generic_map);
                c_generic_map = GenericMap::new(generic_map_type);
            }
            continue
        }
        if empty_lines_counter == -1 {
            // seeds!
            game.set_seeds(c_line.as_str());
            game.set_seeds_part_2(c_line.as_str());
        } else if empty_lines_counter >= 0 {
            if c_line.split_whitespace().count() == 3 {
                // ignore first line describing map type!
                c_generic_map.insert_line(c_line.as_str());
            }
        }
    }
    // also insert last one!
    game.insert_map(c_generic_map);
    println!("Part 1:\t{}", game.play_part_1());
    println!("Part 2:\t{}", game.play_part_2());


    Ok(())
}
use rayon::prelude::*;
use std::{cmp::min, fs};

struct Mapping {
    start: i64,
    dest: i64,
    length: i64,
}

struct MappingList {
    mappings: Vec<Vec<Mapping>>,
}

impl Mapping {
    fn matched(&self, v: i64) -> bool {
        self.start <= v && self.start + self.length - 1 >= v
    }

    fn get_mapped_value(&self, v: i64) -> i64 {
        let offset = v - self.start;
        self.dest + offset
    }
}

impl MappingList {
    fn apply(&self, v: i64) -> i64 {
        let mut cur = v;
        for l in self.mappings.iter() {
            cur = get_mapped_value(cur, l);
        }
        cur
    }
}

fn load_map(lines: Vec<&str>, map: &mut Vec<Mapping>) {
    for line in lines {
        if line.len() < 2 {
            continue;
        }
        let splited: Vec<&str> = line.split_whitespace().collect();
        let dest_st = splited[0].parse::<i64>().expect("");
        let src_st = splited[1].parse::<i64>().expect("");
        let count = splited[2].parse::<i64>().expect("");

        map.push(Mapping {
            start: src_st,
            dest: dest_st,
            length: count,
        });
    }
}

fn get_mapped_value(v: i64, mappings: &Vec<Mapping>) -> i64 {
    for mapping in mappings {
        if mapping.matched(v) {
            return mapping.get_mapped_value(v);
        }
    }
    v
}

fn load_seeds(line: &str, seeds: &mut Vec<i64>) {
    let nums: Vec<&str> = line.split(":").collect();
    let seed_numbers: Vec<&str> = nums[1].split_whitespace().collect();
    for n in seed_numbers {
        seeds.push(n.parse::<i64>().expect(""));
    }
}

fn get_seed_pairs(seeds: &Vec<i64>) -> Vec<(i64, i64)> {
    let mut i = 0;
    let mut result = Vec::new();
    while i < seeds.len() {
        if i % 2 != 1 {
            result.push((seeds[i], seeds[i + 1]));
        }
        i += 1;
    }
    result
}

fn main() {
    let data = fs::read_to_string("data/day5.txt").expect("Cannot read file.");

    let mut seeds: Vec<i64> = Vec::new();
    let mut seeds_soil: Vec<Mapping> = Vec::new();
    let mut soil_fertilizer: Vec<Mapping> = Vec::new();
    let mut fertilizer_water: Vec<Mapping> = Vec::new();
    let mut water_light: Vec<Mapping> = Vec::new();
    let mut light_temperature: Vec<Mapping> = Vec::new();
    let mut temperature_humidity: Vec<Mapping> = Vec::new();
    let mut humidity_location: Vec<Mapping> = Vec::new();

    let lines: Vec<&str> = data.split('\n').collect();
    load_seeds(lines[0], &mut seeds);
    let mut cur_map: &mut Vec<Mapping> = &mut seeds_soil;
    for line in lines.iter().skip(2) {
        if line.len() <= 2 {
            //empty
            continue;
        } else if line.chars().next().unwrap().is_alphabetic() {
            cur_map = match line.trim() {
                "soil-to-fertilizer map:" => &mut soil_fertilizer,
                "fertilizer-to-water map:" => &mut fertilizer_water,
                "water-to-light map:" => &mut water_light,
                "light-to-temperature map:" => &mut light_temperature,
                "temperature-to-humidity map:" => &mut temperature_humidity,
                "humidity-to-location map:" => &mut humidity_location,
                _ => {
                    println!("Should not happen or first!");
                    &mut seeds_soil
                }
            };
        } else {
            //number lines
            load_map(vec![line], cur_map);
        }
    }

    assert!(humidity_location.len() > 0);
    assert!(temperature_humidity.len() > 0);
    assert!(light_temperature.len() > 0);
    assert!(water_light.len() > 0);
    assert!(fertilizer_water.len() > 0);
    assert!(soil_fertilizer.len() > 0);
    assert!(seeds_soil.len() > 0);

    let mapping_list = MappingList {
        mappings: vec![
            seeds_soil,
            soil_fertilizer,
            fertilizer_water,
            water_light,
            light_temperature,
            temperature_humidity,
            humidity_location,
        ],
    };

    let mut min_seed = i64::MAX;
    let seed_pairs: Vec<(i64, i64)> = get_seed_pairs(&seeds);
    for seed in seeds {
        min_seed = min(min_seed, mapping_list.apply(seed));
    }

    println!("p1: {}", min_seed);
    /*
        let mut min_seed_pair_val = i64::MAX;
        let mut min_seed_pair = (i64::MAX, i64::MAX);
        for pair in seed_pairs {
            let first_val = mapping_list.apply(pair.0);
            let last_val = mapping_list.apply(pair.0 + pair.1 - 1);
            if first_val < min_seed_pair_val || last_val < min_seed_pair_val {
                min_seed_pair_val = min(first_val, last_val);
                min_seed_pair = pair;
            }
        }

        println!("pair: {:?}", min_seed_pair);
        let mut min_seed = i64::MAX;
        for i in min_seed_pair.0 .. min_seed_pair.0 + min_seed_pair.1 {
            min_seed = min(min_seed, mapping_list.apply(i));
        }
    */
    /*
        let mut min_seed = i64::MAX;
        for p in seed_pairs {
            for i in p.0 .. p.0 + p.1 {
                min_seed = min(min_seed, mapping_list.apply(i));
            }
        }
    */
    let min_seed = seed_pairs
        .par_iter() // create a parallel iterator over the seed pairs
        .flat_map(|p| p.0..p.0 + p.1) // map each pair to a range and flatten the ranges
        .map(|i| mapping_list.apply(i)) // apply the mapping function to each element
        .min_by(|x, y| x.cmp(y)) // find the minimum element using the cmp method
        .unwrap_or(i64::MAX);
    println!("p2: {}", min_seed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_map() {
        let lines = vec!["50 98 2", "52 50 48"];
        let mut result: Vec<Mapping> = Vec::new();

        load_map(lines, &mut result);
        assert_eq!(get_mapped_value(1, &result), 1); //passthrough
        assert_eq!(get_mapped_value(49, &result), 49);
        assert_eq!(get_mapped_value(50, &result), 52);
        assert_eq!(get_mapped_value(53, &result), 55);
        assert_eq!(get_mapped_value(97, &result), 99);
        assert_eq!(get_mapped_value(98, &result), 50);
        assert_eq!(get_mapped_value(99, &result), 51);
        assert_eq!(get_mapped_value(100, &result), 100);
    }

    #[test]
    fn test_load_seeds() {
        let line = "seeds: 79 14 55 13";
        let mut seeds: Vec<i64> = Vec::new();
        load_seeds(line, &mut seeds);
        assert!(seeds.len() == 4);
        assert_eq!(seeds[0], 79);
        assert_eq!(seeds[1], 14);
        assert_eq!(seeds[2], 55);
        assert_eq!(seeds[3], 13);
    }
}

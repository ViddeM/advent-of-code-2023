use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Instructions {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

#[derive(Debug, Clone)]
pub struct Map {
    source_name: String,
    dest_name: String,
    map_ranges: Vec<MapRange>,
}

#[derive(Debug, Clone)]
pub struct MapRange {
    diff: i64,
    source_start: i64,
    dest_start: i64,
    length: i64,
}

impl MapRange {
    fn contains_source(&self, source: i64) -> bool {
        source >= self.source_start && source < self.source_start + self.length
    }
}

pub fn parse<'a>(input: &str) -> Instructions {
    let (seed_line, input) = input.split_once("\n").unwrap();
    let seeds = seed_line
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let maps = input
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .map(|map_text| {
            let (name, rest) = map_text.split_once("\n").expect("Failed to split map_text");

            let (source, dest) = name
                .strip_suffix(" map:")
                .unwrap()
                .split_once("-to-")
                .unwrap();

            let map_ranges = rest
                .lines()
                .map(|numbers| {
                    let (dest_start, rest) = numbers.split_once(" ").unwrap();
                    let (source_start, length) = rest.split_once(" ").unwrap();

                    let s = source_start.parse().unwrap();
                    let d = dest_start.parse().unwrap();
                    let diff = d - s;

                    MapRange {
                        diff,
                        source_start: s,
                        dest_start: d,
                        length: length.parse().unwrap(),
                    }
                })
                .collect();

            Map {
                source_name: source.to_string(),
                dest_name: dest.to_string(),
                map_ranges,
            }
        })
        .collect();

    Instructions { seeds, maps }
}

pub fn solve_part_one<'a>(input: Instructions) -> String {
    let mut seed_to_location_map: HashMap<i64, Vec<i64>> = HashMap::new();

    for seed in input.seeds.iter() {
        println!("Beginning with seed {seed}");

        let mut source_name = "seed";
        let mut curr_val = seed.clone();

        seed_to_location_map.insert(seed.clone(), vec![]);

        'search: while source_name != "location" {
            println!("\tWorking with source {source_name}...");
            for map in input.maps.iter() {
                if map.source_name == source_name {
                    let map_range = map
                        .map_ranges
                        .iter()
                        .filter(|range| range.contains_source(curr_val))
                        .collect::<Vec<&MapRange>>();

                    if map_range.len() > 1 {
                        // In case I was wrong :(
                        panic!("Found a map with more than 1 matching range!");
                    }

                    if map_range.is_empty() {
                        // Found no matching map range, keeping the same val.
                        source_name = &map.dest_name;
                        seed_to_location_map
                            .get_mut(seed)
                            .expect("SEED NOT FOUND IN MAP!")
                            .push(curr_val);
                        continue 'search;
                    }

                    curr_val = curr_val + map_range[0].diff;
                    seed_to_location_map
                        .get_mut(seed)
                        .expect("SEED NOT FOUND IN MAP!")
                        .push(curr_val);

                    source_name = &map.dest_name;
                    continue 'search;
                }
            }
        }
    }

    let mut locations = seed_to_location_map
        .into_iter()
        .map(|(_, path)| path.last().unwrap().clone())
        .collect::<Vec<i64>>();

    locations.sort();

    locations.first().unwrap().to_string()
}

pub fn solve_part_two<'a>(input: Instructions) -> String {
    let mut seed_to_location_map: HashMap<i64, Vec<i64>> = HashMap::new();

    let mut seeds = vec![];

    for i in (0..input.seeds.len()).step_by(2) {
        let seed = input.seeds[i];
        let len = input.seeds[i + 1];

        for s in (seed..seed + len) {
            seeds.push(s);
        }
    }

    let mut min_found = i64::MAX;

    for seed in seeds.iter() {
        let mut curr_val = seed.clone();

        'map: for map in &input.maps {
            for range in &map.map_ranges {
                if range.contains_source(curr_val) {
                    curr_val = curr_val + range.diff;
                    continue 'map;
                }
            }
        }

        if curr_val < min_found {
            min_found = curr_val;
        }
    }

    min_found.to_string()
}

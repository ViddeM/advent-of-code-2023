use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum C {
    Operational,
    Broken,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Line {
    springs: Vec<C>,
    operationals: Vec<usize>,
}

pub fn parse<'a>(input: &'a str) -> &'a str {
    input
}

fn parse_part_one<'a>(input: &'a str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let (map, info) = l.split_once(" ").unwrap();
            let springs = map
                .chars()
                .map(|c| match c {
                    '?' => C::Unknown,
                    '#' => C::Operational,
                    '.' => C::Broken,
                    _ => panic!("Invalid input char {c}"),
                })
                .collect();

            let operationals = info
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            Line {
                springs,
                operationals,
            }
        })
        .collect()
}

fn matches_info(map: &Vec<&C>, info: &Vec<usize>) -> bool {
    let mut index = 0;

    let st = map
        .iter()
        .map(|c| match c {
            C::Operational => '#',
            C::Broken => '.',
            C::Unknown => panic!("SHouldn't contain unknowns anymore"),
        })
        .fold(String::new(), |mut acc, elem| {
            acc.push(elem);
            acc
        });

    let nums = st
        .split(".")
        .filter(|c| !c.is_empty())
        .map(|c| c.len())
        .collect::<Vec<usize>>();

    &nums == info
}

pub fn solve_part_one<'a>(input: &'a str) -> String {
    let input = parse_part_one(input);

    let mut sum = 0;
    for line in input.iter() {
        let mut alts: Vec<Vec<&C>> =
            Vec::with_capacity(1 << line.springs.iter().filter(|c| c == &&C::Unknown).count());
        alts.push(Vec::with_capacity(line.springs.len()));

        for spring in line.springs.iter() {
            if spring == &C::Unknown {
                alts.iter_mut().for_each(|list| list.push(&C::Operational));

                for i in 0..alts.len() {
                    let mut new = alts[i].clone();
                    new[alts[i].len() - 1] = &C::Broken;
                    alts.push(new);
                }
            } else {
                alts.iter_mut().for_each(|list| list.push(spring));
            }
        }

        // Filter away those that doesn't match our expected numbers.
        sum += alts
            .into_iter()
            .filter(|c| matches_info(c, &line.operationals))
            .count();
    }

    sum.to_string()
}

fn parse_part_two<'a>(input: &'a str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let (map, info) = l.split_once(" ").unwrap();

            let mut n_map = String::with_capacity(map.len() * 5 + 5);
            let mut n_info = String::with_capacity(info.len() * 5 + 4);
            for i in 0..5 {
                n_map.push_str(map);
                n_info.push_str(info);
                if i == 4 {
                    n_map.push('.');
                } else {
                    n_info.push(',');
                    n_map.push('?');
                }
            }

            let springs = n_map
                .chars()
                .map(|c| match c {
                    '?' => C::Unknown,
                    '#' => C::Operational,
                    '.' => C::Broken,
                    _ => panic!("Invalid input char {c}"),
                })
                .collect();

            let operationals = n_info
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            Line {
                springs,
                operationals,
            }
        })
        .collect()
}

pub fn solve_part_two<'a>(input: &'a str) -> String {
    let input = parse_part_two(input);

    let mut sum = 0;
    for line in input.iter() {
        let mut map = HashMap::new();
        sum += count(&mut map, &line.springs, &line.operationals, 0, 0, 0);
    }

    sum.to_string()
}

fn count(
    previous: &mut HashMap<(usize, usize, usize), usize>,
    springs: &Vec<C>,
    info: &Vec<usize>,
    index: usize,
    operational_index: usize,
    finished_operationals: usize,
) -> usize {
    let key = (index, operational_index, finished_operationals);

    if previous.contains_key(&key) {
        return previous[&key].clone();
    }

    let mut val: usize = 0;
    if index == springs.len() {
        // We've gone through them all and found no alternatives.
        if info.len() == finished_operationals {
            val = 1;
        }
    } else if springs[index] == C::Operational {
        // We found an operational
        val = count(
            previous,
            springs,
            info,
            index + 1,
            operational_index + 1,
            finished_operationals,
        );
    } else if springs[index] == C::Broken || finished_operationals == info.len() {
        // We found a broken one OR we've reached the end of the numbers.
        if finished_operationals < info.len() && operational_index == info[finished_operationals] {
            val = count(
                previous,
                springs,
                info,
                index + 1,
                0,
                finished_operationals + 1,
            );
        } else if operational_index == 0 {
            val = count(previous, springs, info, index + 1, 0, finished_operationals);
        } else {
            val = 0;
        }
    } else {
        // We have an unknown
        let num_operational = count(
            previous,
            springs,
            info,
            index + 1,
            operational_index + 1,
            finished_operationals,
        );
        let num_broken = if operational_index == info[finished_operationals] {
            count(
                previous,
                springs,
                info,
                index + 1,
                0,
                finished_operationals + 1,
            )
        } else if operational_index == 0 {
            count(previous, springs, info, index + 1, 0, finished_operationals)
        } else {
            0
        };

        val = num_operational + num_broken;
    }

    previous.insert(key, val);

    val
}

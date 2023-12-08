use std::collections::HashMap;

pub struct Map {
    instructions: Vec<Dir>,
    map: HashMap<(String, Dir), String>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
}

pub fn parse<'a>(input: &str) -> Map {
    let (lr_ins, map) = input.split_once("\n\n").unwrap();

    let instructions = lr_ins
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Invalid instruction {c}"),
        })
        .collect::<Vec<Dir>>();

    let mut map_map = HashMap::new();
    for l in map.lines() {
        let (start, end) = l.split_once(" = ").unwrap();
        let (left, right) = end
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_once(", ")
            .unwrap();

        map_map.insert((start.to_string(), Dir::Left), left.to_string());
        map_map.insert((start.to_string(), Dir::Right), right.to_string());
    }

    Map {
        instructions,
        map: map_map,
    }
}

fn get_steps_for_start(start: String, input: &Map, part1: bool) -> i64 {
    let mut curr = start.clone();
    let mut ins_index = 0;
    let mut steps = 0;

    while (part1 && curr != "ZZZ") || (!part1 && !curr.ends_with("Z")) {
        let ins = input.instructions[ins_index].clone();
        ins_index = (ins_index + 1) % input.instructions.len();

        let dir = input
            .map
            .get(&(curr, ins))
            .expect("Couldn't find step in map :(");

        curr = dir.clone();
        steps += 1;
    }

    steps
}

pub fn solve_part_one<'a>(input: Map) -> String {
    let steps = get_steps_for_start(String::from("AAA"), &input, true);

    steps.to_string()
}

pub fn solve_part_two<'a>(input: Map) -> String {
    let starts = input
        .map
        .keys()
        .map(|(s, _)| s)
        .filter(|s| s.ends_with("A"))
        .cloned()
        .collect::<Vec<String>>();
    let mut steps = 1;

    for s in starts {
        let curr_steps = get_steps_for_start(s, &input, false);
        steps = num::integer::lcm(steps, curr_steps);
    }

    steps.to_string()
}

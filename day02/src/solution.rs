use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub struct Game {
    id: u32,
    sets: Vec<HashMap<Color, u32>>,
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Game> + 'a {
    input.lines().map(|l| {
        let l = l
            .strip_prefix("Game ")
            .expect("Failed to strip prefix Game ");
        let (id, sets) = l.split_once(": ").expect("Failed to split games with sets");
        let id = id.parse::<u32>().expect("Failed to parse ID");

        let sets = sets
            .split("; ")
            .map(|s| {
                s.split(", ")
                    .map(|entry| {
                        let (count, color) = entry
                            .split_once(" ")
                            .expect("Failed to split on space in set");
                        let color = match color {
                            "green" => Color::Green,
                            "red" => Color::Red,
                            "blue" => Color::Blue,
                            _ => panic!("Invalid color {color}"),
                        };

                        let count = count.parse::<u32>().expect("Failed to parse entry count");

                        (color, count)
                    })
                    .collect::<HashMap<Color, u32>>()
            })
            .collect();

        Game { id, sets }
    })
}

const POSSIBLE_RED: u32 = 12;
const POSSIBLE_GREEN: u32 = 13;
const POSSIBLE_BLUE: u32 = 14;

pub fn solve_part_one<'a>(input: impl Iterator<Item = Game>) -> String {
    input
        .filter(|game| game_is_possible(game))
        .map(|game| game.id)
        .sum::<u32>()
        .to_string()
}

fn game_is_possible(game: &Game) -> bool {
    !game.sets.iter().any(|set| {
        let reds = set.get(&Color::Red).unwrap_or(&0);
        let blues = set.get(&Color::Blue).unwrap_or(&0);
        let greens = set.get(&Color::Green).unwrap_or(&0);

        reds > &POSSIBLE_RED || greens > &POSSIBLE_GREEN || blues > &POSSIBLE_BLUE
    })
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = Game>) -> String {
    input
        .map(|game| power_of_min_number_of_cubes_for_sets(&game))
        .sum::<u32>()
        .to_string()
}

fn power_of_min_number_of_cubes_for_sets(game: &Game) -> u32 {
    let mut max_reds: u32 = 0;
    let mut max_blues: u32 = 0;
    let mut max_greens: u32 = 0;

    for (&reds, &blues, &greens) in game.sets.iter().map(|set| {
        let reds = set.get(&Color::Red).unwrap_or(&0);
        let blues = set.get(&Color::Blue).unwrap_or(&0);
        let greens = set.get(&Color::Green).unwrap_or(&0);

        (reds, blues, greens)
    }) {
        if reds > max_reds {
            max_reds = reds;
        }
        if blues > max_blues {
            max_blues = blues;
        }
        if greens > max_greens {
            max_greens = greens;
        }
    }

    max_reds * max_blues * max_greens
}

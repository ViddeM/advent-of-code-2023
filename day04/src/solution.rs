use std::collections::{HashMap, HashSet};

pub struct Card {
    id: u32,
    winning_cards: HashSet<u32>,
    my_cards: HashSet<u32>,
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Card> + 'a {
    input.lines().map(|l| {
        let (card, rest) = l.split_once(": ").expect("Failed to split on : ");
        let card_num = card
            .strip_prefix("Card ")
            .expect("Failed to remove card prefix");

        let (winning, mine) = rest.split_once(" | ").expect("Failed to split on PIPE");
        let win = winning
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|w| w.parse::<u32>().expect("Failed to parse winning num"))
            .collect::<HashSet<u32>>();

        let mine = mine
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|w| w.parse::<u32>().expect("Failed to parse my num"))
            .collect::<HashSet<u32>>();

        Card {
            id: card_num
                .trim()
                .parse::<u32>()
                .expect("Failed to parse card ID"),
            winning_cards: win,
            my_cards: mine,
        }
    })
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = Card>) -> String {
    input
        .map(|card| {
            card.my_cards
                .iter()
                .filter(|my| card.winning_cards.contains(my))
                .count()
        })
        .filter(|n| n > &0)
        .map(|num| 1 << num - 1)
        .sum::<u32>()
        .to_string()
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = Card>) -> String {
    let mut duplication_map: HashMap<u32, u32> = HashMap::new();
    let mut count = 0;

    for card in input {
        let copies = duplication_map.get(&card.id).unwrap_or(&0) + 1;
        count += copies;

        let num_matching = card
            .my_cards
            .iter()
            .filter(|my| card.winning_cards.contains(my))
            .count();

        for inc in card.id + 1..card.id + 1 + num_matching as u32 {
            let num = duplication_map.get(&inc).unwrap_or(&0) + copies;
            duplication_map.insert(inc, num);
        }
    }

    count.to_string()
}

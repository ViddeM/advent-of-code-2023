use std::collections::HashMap;

#[derive(PartialEq, PartialOrd)]
pub struct Hand {
    hand: Vec<char>,
    bid: u32,
}

#[derive(PartialEq)]
pub enum Type {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Type {
    fn get_val(&self) -> u32 {
        match self {
            Type::FiveOfKind => 7,
            Type::FourOfKind => 6,
            Type::FullHouse => 5,
            Type::ThreeOfKind => 4,
            Type::TwoPair => 3,
            Type::OnePair => 2,
            Type::HighCard => 1,
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_val().partial_cmp(&other.get_val())
    }
}

impl Hand {
    pub fn get_type(&self) -> Type {
        let map = self.hand.iter().fold(HashMap::new(), |mut acc, c| {
            let n = acc.remove(c).unwrap_or(0) + 1;
            acc.insert(*c, n);
            acc
        });

        let different = map.len();
        let count = map.values().max().unwrap();

        match (different, count) {
            (5, _) => Type::HighCard,
            (4, _) => Type::OnePair,
            (3, 2) => Type::TwoPair,
            (3, 3) => Type::ThreeOfKind,
            (2, 3) => Type::FullHouse,
            (2, 4) => Type::FourOfKind,
            (1, 5) => Type::FiveOfKind,
            _ => panic!("Uncovered pattern {different} different and {count} highest count"),
        }
    }

    pub fn get_type_2(&self) -> Type {
        let map = self.hand.iter().fold(HashMap::new(), |mut acc, c| {
            let n = acc.remove(c).unwrap_or(0) + 1;
            acc.insert(*c, n);
            acc
        });

        let num_jokers = map.get(&'J').unwrap_or(&0).clone();

        let different = map.len() - if num_jokers > 0 { 1 } else { 0 };
        let count = map
            .keys()
            .filter(|c| c != &&'J')
            .map(|k| map.get(k).unwrap())
            .max()
            .unwrap_or(&0)
            + num_jokers;

        match (different, count) {
            (5, _) => Type::HighCard,
            (4, _) => Type::OnePair,
            (3, 2) => Type::TwoPair,
            (3, 3) => Type::ThreeOfKind,
            (2, 3) => Type::FullHouse,
            (2, 4) => Type::FourOfKind,
            (1, 5) => Type::FiveOfKind,
            (0, 5) => Type::FiveOfKind, // All Jokers
            _ => panic!("Uncovered pattern {different} different and {count} highest count"),
        }
    }
}

fn char_val(c: &char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Invalid char '{c}'"),
    }
}

fn char_val_2(c: &char) -> u32 {
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => panic!("Invalid char '{c}'"),
    }
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Hand> + 'a {
    input.lines().map(|l| {
        let (hand, bid) = l.split_once(" ").expect("Failed to split");

        Hand {
            hand: hand.chars().collect(),
            bid: bid.parse().expect("Failed to parse bid"),
        }
    })
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = Hand>) -> String {
    let mut inp = input.collect::<Vec<Hand>>();
    inp.sort_by(|h1, h2| {
        let t1 = h1.get_type();
        let t2 = h2.get_type();

        if t1 == t2 {
            for (i, c1) in h1.hand.iter().enumerate() {
                let v1 = char_val(c1);
                let v2 = char_val(&h2.hand[i]);

                if v1 == v2 {
                    continue;
                }

                return v1.cmp(&v2);
            }
        }

        return t1.partial_cmp(&t2).unwrap();
    });

    inp.iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid as usize)
        .sum::<usize>()
        .to_string()
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = Hand>) -> String {
    let mut inp = input.collect::<Vec<Hand>>();
    inp.sort_by(|h1, h2| {
        let t1 = h1.get_type_2();
        let t2 = h2.get_type_2();

        if t1 == t2 {
            for (i, c1) in h1.hand.iter().enumerate() {
                let v1 = char_val_2(c1);
                let v2 = char_val_2(&h2.hand[i]);

                if v1 == v2 {
                    continue;
                }

                return v1.cmp(&v2);
            }
        }

        return t1.partial_cmp(&t2).unwrap();
    });

    inp.iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid as usize)
        .sum::<usize>()
        .to_string()
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    input.lines()
}

const NUMBERS_MAP_1: [(&str, u32); 9] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

pub fn solve_part_one<'a>(input: impl Iterator<Item = &'a str>) -> String {
    input
        .map(|l| {
            let mut l = l;
            let first = 'l: loop {
                for (num, val) in NUMBERS_MAP_1.iter() {
                    if l.starts_with(num) {
                        break 'l val;
                    }
                }
                // Move pointer one step forward.
                l = &l[1..];
            };

            let second = 'l: loop {
                for (num, val) in NUMBERS_MAP_1.iter() {
                    if l.starts_with(num) {
                        break 'l val;
                    }
                }
                // Move pointer one step back.
                l = &l[..l.len() - 1];
            };

            first * 10 + second
        })
        .sum::<u32>()
        .to_string()
}

const NUMBERS_MAP_2: [(&str, u32); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn solve_part_two<'a>(input: impl Iterator<Item = &'a str>) -> String {
    input
        .map(|l| {
            let mut l = l;
            let first = 'l: loop {
                for (num, val) in NUMBERS_MAP_2.iter() {
                    if l.starts_with(num) {
                        break 'l val;
                    }
                }
                // Move pointer one step forward.
                l = &l[1..];
            };

            let second = 'l: loop {
                for (num, val) in NUMBERS_MAP_2.iter() {
                    if l.ends_with(num) {
                        break 'l val;
                    }
                }
                // Move pointer one step back.
                l = &l[..l.len() - 1];
            };

            first * 10 + second
        })
        .sum::<u32>()
        .to_string()
}

use std::collections::HashMap;

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    input.lines()
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = &'a str>) -> String {
    let input = input.map(|l| {
        let nums = l
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        let first = nums.first().unwrap();

        let second = nums.last().unwrap();

        first * 10 + second
    });

    let sum: u32 = input.sum();
    sum.to_string()
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = &'a str>) -> String {
    let numbers_map: HashMap<&str, u32> = HashMap::from([
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
    ]);

    input
        .map(|l| {
            let mut l = l;
            let first = 'l: loop {
                for (num, val) in numbers_map.iter() {
                    if l.starts_with(num) {
                        break 'l val;
                    }
                }
                // Remove first digit
                l = &l[1..];
            };

            let second = 'l: loop {
                for (num, val) in numbers_map.iter() {
                    if l.ends_with(num) {
                        break 'l val;
                    }
                }
                // Remove last digit
                l = &l[..l.len() - 1];
            };

            first * 10 + second
        })
        .sum::<u32>()
        .to_string()
}

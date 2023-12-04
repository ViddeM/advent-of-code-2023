use std::collections::HashMap;

const fn recode_char(b: &u8) -> u8 {
    match b {
        b' ' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        _ => 0,
    }
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    let start = input.find(':').unwrap();
    let middle = input.find('|').unwrap();

    input.lines().map(move |l| {
        let first = l[start + 1..middle]
            .as_bytes()
            .iter()
            .map(recode_char)
            .array_chunks::<3>()
            .map(|chunk| chunk[1] * 10 + chunk[2])
            .collect::<Vec<u8>>();

        l[middle + 1..]
            .as_bytes()
            .iter()
            .map(recode_char)
            .array_chunks::<3>()
            .map(|chunk| chunk[1] * 10 + chunk[2])
            .filter(|n| first.contains(n))
            .count()
    })
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = usize>) -> String {
    input
        .filter(|n| n > &0)
        .map(|num| 1 << num - 1)
        .sum::<u32>()
        .to_string()
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = usize>) -> String {
    let mut duplication_map: HashMap<usize, u32> = HashMap::new();
    let mut count = 0;

    for (index, card) in input.enumerate() {
        let copies = duplication_map.get(&index).unwrap_or(&0) + 1;
        count += copies;

        let num_matching = card;

        for inc in index + 1..index + 1 + num_matching {
            let num = duplication_map.get(&inc).unwrap_or(&0) + copies;
            duplication_map.insert(inc, num);
        }
    }

    count.to_string()
}

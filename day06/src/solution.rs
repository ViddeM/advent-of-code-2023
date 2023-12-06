pub fn parse<'a>(input: &'a str) -> &'a str {
    input
}

pub fn solve_part_one<'a>(input: &'a str) -> String {
    let (time, distance) = input.split_once("\n").expect("No newline?");
    let time_iter = time
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split(" ")
        .filter(|l| !l.is_empty())
        .map(|n| n.parse::<u32>().unwrap());
    let dist_iter = distance
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split(" ")
        .filter(|l| !l.is_empty())
        .map(|n| n.parse::<u32>().unwrap());

    let input = std::iter::zip(time_iter, dist_iter);

    input
        .map(|(time, dist)| {
            (0..=time)
                .map(|t| (time - t) * t)
                .filter(|d| d > &dist)
                .count()
        })
        .fold(1, |acc, count| count * acc)
        .to_string()
}

pub fn solve_part_two<'a>(input: &'a str) -> String {
    let (time, distance) = input.split_once("\n").expect("No newline?");
    let time = time
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let dist = distance
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .expect("Failed to parse dist");

    (0..=time)
        .map(|t| (time - t) * t)
        .filter(|d| d > &dist)
        .count()
        .to_string()
}

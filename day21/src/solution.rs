use std::collections::HashSet;

pub fn parse<'a>(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn solve_part_one<'a>(input: Vec<Vec<char>>) -> String {
    let (start_x, start_y) = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| c == &&'S')
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<(usize, usize)>>()
        .first()
        .unwrap()
        .clone();

    let width = input[0].len();
    let height = input.len();

    let mut last_turn: Vec<(usize, usize)> = vec![(start_x, start_y)];

    for step in 0..=64 {
        let mut next_turn = HashSet::new();

        while let Some((x, y)) = last_turn.pop() {
            let mut neighs = vec![];
            if x > 0 {
                neighs.push((x - 1, y));
            }
            if x < width - 1 {
                neighs.push((x + 1, y));
            }
            if y > 0 {
                neighs.push((x, y - 1));
            }
            if y < height - 1 {
                neighs.push((x, y + 1));
            }

            neighs
                .into_iter()
                .filter(|(x, y)| input[*y][*x] != '#')
                .for_each(|(x, y)| {
                    next_turn.insert((x, y));
                });
        }

        last_turn = next_turn.into_iter().collect();
    }

    last_turn.len().to_string()
}

pub fn solve_part_two<'a>(input: Vec<Vec<char>>) -> String {
    let (start_x, start_y) = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| c == &&'S')
                .map(move |(x, _)| (x as i128, y as i128))
        })
        .collect::<Vec<(i128, i128)>>()
        .first()
        .unwrap()
        .clone();

    let width = input[0].len() as i128;
    let height = input.len() as i128;

    let mut half_map_vals = vec![];
    let half_map = ((width - 1) / 2) as usize;

    let mut last_turn: Vec<(i128, i128)> = vec![(start_x, start_y)];

    for step in 1..=((width * 2) as usize + half_map) {
        let mut next_turn = HashSet::new();

        while let Some((x, y)) = last_turn.pop() {
            [(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter(|(x, y)| {
                    input[(((y % height) + height) % height) as usize]
                        [(((x % width) + height) % height) as usize]
                        != '#'
                })
                .for_each(|(x, y)| {
                    next_turn.insert((x, y));
                });
        }

        last_turn = next_turn.into_iter().collect();
        if step >= 65 && (step - half_map) % width as usize == 0 {
            println!("ADDING FOR STEP {step} -- {}", last_turn.len());
            half_map_vals.push(last_turn.len());
        }
    }

    println!("HALF_MAP_VALS:: {half_map_vals:?}");

    assert!(half_map_vals.len() == 3);
    // let delta = half_map_vals[2] - half_map_vals[1];
    // vals repeat every 65 + 131 steps,
    // use this to get a polynomial that can be used to calculate the answer

    // Shamelessly use wolfram alpha to get the answer.

    format!("Shamelessly use the above values in wolfram alpha to get the answer")
}

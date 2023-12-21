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
        if step >= half_map && (step - half_map) % width as usize == 0 {
            half_map_vals.push(last_turn.len());
        }
    }

    assert!(half_map_vals.len() == 3);

    let hm = half_map as i128;
    let hm_vals = half_map_vals
        .into_iter()
        .map(|v| v as i128)
        .collect::<Vec<i128>>();

    find_polynomial_value_at(
        [
            (hm, hm_vals[0]),
            (hm + width, hm_vals[1]),
            (hm + width * 2, hm_vals[2]),
        ],
        26501365,
    )
    .to_string()
}

fn find_polynomial_value_at(points: [(i128, i128); 3], x: i128) -> i128 {
    let (a, b, c, denominator) = find_coefficients(points);

    let x_sq = x.pow(2);
    let ax_sq = a * x_sq;

    let bx = b * x;

    let ans_numerator = ax_sq + bx + c;
    ans_numerator / denominator
}

fn find_coefficients(points: [(i128, i128); 3]) -> (i128, i128, i128, i128) {
    let [(x1, y1), (x2, y2), (x3, y3)] = points;

    // Using Cramer's rule to solve the system of linear equations
    let denominator = -(x1 - x2) * (x1 - x3) * (x2 - x3);

    if denominator == 0 {
        panic!("Failed to find coefficients!");
    }

    let a = (y1 * (x2 - x3) + y2 * (x3 - x1) + y3 * (x1 - x2)) * -1;
    let b = y1 * (x2 + x3) * (x2 - x3) + y2 * (x3 + x1) * (x3 - x1) + y3 * (x1 + x2) * (x1 - x2);
    let c = (y1 * x2 * x3 * (x2 - x3) + y2 * x3 * x1 * (x3 - x1) + y3 * x1 * x2 * (x1 - x2)) * -1;

    (a, b, c, denominator)
}

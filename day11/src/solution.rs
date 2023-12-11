use std::collections::HashSet;

pub fn parse<'a>(input: &'a str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x.clone(), y.clone()))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect()
}

fn find_min_max(map: &Vec<(usize, usize)>) -> (usize, usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;

    for (x, y) in map.iter() {
        if x > &max_x {
            max_x = *x;
        }

        if y > &max_y {
            max_y = *y;
        }

        if x < &min_x {
            min_x = *x;
        }

        if y < &min_y {
            min_y = *y;
        }
    }

    (min_x, max_x, min_y, max_y)
}

fn expand_map(map: &Vec<(usize, usize)>, expand_size: usize) -> Vec<(usize, usize)> {
    let (min_x, max_x, min_y, max_y) = find_min_max(map);
    let mut empty_cols = HashSet::with_capacity(max_x - min_x);
    let mut empty_rows = HashSet::with_capacity(max_y - min_y);

    for x in min_x..max_x {
        empty_cols.insert(x);
    }
    for y in min_y..max_y {
        empty_rows.insert(y);
    }

    for (x, y) in map.iter() {
        empty_cols.remove(x);
        empty_rows.remove(y);
    }

    map.into_iter()
        .map(|(x, y)| {
            let cols_to_inc = (min_x..*x).filter(|x| empty_cols.contains(x)).count() * expand_size;
            let rows_to_inc = (min_y..*y).filter(|y| empty_rows.contains(y)).count() * expand_size;

            (x + cols_to_inc, y + rows_to_inc)
        })
        .collect()
}

pub fn solve_part_one<'a>(input: Vec<(usize, usize)>) -> String {
    let map = expand_map(&input, 1);

    let mut finished_pairs = HashSet::with_capacity(map.capacity());
    let mut sum = 0;

    for (first_x, first_y) in map.iter() {
        for (second_x, second_y) in map.iter() {
            if first_x == second_x && first_y == second_y {
                // Skip ourselves.
                continue;
            }

            if finished_pairs.contains(&(*second_x, *second_y)) {
                // Handled in a previous iteration
                continue;
            }

            let length = first_x.abs_diff(*second_x) + first_y.abs_diff(*second_y);
            sum += length;
        }

        finished_pairs.insert((*first_x, *first_y));
    }

    sum.to_string()
}

pub fn solve_part_two<'a>(input: Vec<(usize, usize)>) -> String {
    let map = expand_map(&input, 1_000_000 - 1);

    let mut finished_pairs = HashSet::with_capacity(map.capacity());
    let mut sum = 0;

    for (first_x, first_y) in map.iter() {
        for (second_x, second_y) in map.iter() {
            if first_x == second_x && first_y == second_y {
                // Skip ourselves.
                continue;
            }

            if finished_pairs.contains(&(*second_x, *second_y)) {
                // Handled in a previous iteration
                continue;
            }

            let length = first_x.abs_diff(*second_x) + first_y.abs_diff(*second_y);
            sum += length;
        }

        finished_pairs.insert((*first_x, *first_y));
    }

    sum.to_string()
}

use std::collections::HashSet;

pub fn parse<'a>(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_longest(
    map: &Vec<Vec<char>>,
    path: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Option<usize> {
    let mut neighbours = vec![];
    match map[y][x] {
        'G' => return Some(path.len() - 1),
        '>' => neighbours.push((x + 1, y)),
        '^' => neighbours.push((x, y - 1)),
        '<' => neighbours.push((x - 1, y)),
        'v' => neighbours.push((x, y + 1)),
        '.' => {
            if x > 0 {
                neighbours.push((x - 1, y));
            }
            if y > 0 {
                neighbours.push((x, y - 1));
            }
            if x < width - 1 {
                neighbours.push((x + 1, y));
            }
            if y < height - 1 {
                neighbours.push((x, y + 1));
            }
        }
        '#' => return None,
        c => panic!("On invalid char {c}"),
    }

    let mut length = None;
    for neigh in neighbours.iter() {
        if path.contains(neigh) {
            // Already checked it
            continue;
        }

        path.insert(*neigh);
        let (neigh_x, neigh_y) = neigh;
        let neigh_len = find_longest(map, path, *neigh_x, *neigh_y, width, height);
        path.remove(neigh);
        if let Some(l) = neigh_len {
            let is_better = if let Some(l2) = length { l > l2 } else { true };

            if is_better {
                length = Some(l);
            }
        }
    }
    length
}

pub fn solve_part_one<'a>(mut input: Vec<Vec<char>>) -> String {
    let start_pos = input[0]
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&'.')
        .map(|(x, _)| (x, 0))
        .collect::<Vec<(usize, usize)>>()[0];

    let width = input[0].len();
    let height = input.len();

    let (end_x, end_y) = input[height - 1]
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&'.')
        .map(|(x, _)| (x, height - 1))
        .collect::<Vec<(usize, usize)>>()[0];

    // Mark the goal as 'G'
    input[end_y][end_x] = 'G';

    let mut path = HashSet::new();
    path.insert(start_pos);

    find_longest(&input, &mut path, start_pos.0, start_pos.1, width, height)
        .unwrap()
        .to_string()
}

fn find_longest_p2(
    map: &Vec<Vec<char>>,
    path: &mut Vec<Vec<bool>>,
    longest: &mut usize,
    length: usize,
    x: usize,
    y: usize,
) {
    let neighbours = match map[y][x] {
        'G' => {
            if &length > longest {
                *longest = length;
            }
            return;
        }
        '#' => return,
        _ => [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice(),
    };

    for (delta_x, delta_y) in neighbours {
        let neigh_x = ((x as isize) + delta_x) as usize;
        let neigh_y = ((y as isize) + delta_y) as usize;
        let Some(neigh_tile) = map.get(neigh_y).and_then(|r| r.get(neigh_x)) else {
            continue;
        };

        if path[neigh_y][neigh_x] || neigh_tile == &'#' {
            // Already checked it
            continue;
        }

        path[neigh_y][neigh_x] = true;
        find_longest_p2(map, path, longest, length + 1, neigh_x, neigh_y);
        path[neigh_y][neigh_x] = false;
    }
}

pub fn solve_part_two<'a>(mut input: Vec<Vec<char>>) -> String {
    let start_pos = input[0]
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&'.')
        .map(|(x, _)| (x, 0))
        .collect::<Vec<(usize, usize)>>()[0];

    let width = input[0].len();
    let height = input.len();

    let (end_x, end_y) = input[height - 1]
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&'.')
        .map(|(x, _)| (x, height - 1))
        .collect::<Vec<(usize, usize)>>()[0];

    // Mark the goal as 'G'
    input[end_y][end_x] = 'G';

    let mut visited = vec![vec![false; width]; height];
    visited[start_pos.1][start_pos.0] = true;

    let mut longest = 0;

    find_longest_p2(
        &input,
        &mut visited,
        &mut longest,
        0,
        start_pos.0,
        start_pos.1,
    );

    longest.to_string()
}

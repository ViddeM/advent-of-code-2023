use std::collections::HashMap;

pub fn parse<'a>(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

pub fn solve_part_one<'a>(input: Vec<Vec<u8>>) -> String {
    let mut new_map: Vec<Vec<u8>> = Vec::with_capacity(input.len());
    let mut sum = 0;

    for y in 0..input.len() {
        let row = &input[y];
        new_map.push(row.clone());
        for x in 0..row.len() {
            let col = &row[x];
            match col {
                b'O' => {
                    // Roll northwards.
                    let mut new_y = y;
                    for roll_y in (0..y).rev() {
                        if new_map[roll_y][x] != b'.' {
                            break;
                        }

                        new_y = roll_y;
                    }

                    new_map[y][x] = b'.';
                    new_map[new_y][x] = b'O';

                    sum += input.len() - new_y;
                }
                b'.' => {
                    new_map[y][x] = b'.';
                }
                b'#' => {
                    new_map[y][x] = b'#';
                }
                _ => panic!("Invalid char {col}"),
            }
        }
    }

    sum.to_string()
}

const ROUNDS: usize = 1_000_000_000;

pub fn solve_part_two<'a>(input: Vec<Vec<u8>>) -> String {
    let mut prev_maps: HashMap<Vec<Vec<u8>>, usize> = HashMap::new();

    let mut map = input;
    let mut cycle_start_index = 0;
    let mut cycle_end_index = 0;

    for i in 0..ROUNDS {
        if let Some(c) = prev_maps.get(&map) {
            println!("State at index {i} is a copy of state at index {c}");
            cycle_start_index = *c;
            cycle_end_index = i;
            break;
        }

        prev_maps.insert(map.clone(), i);
        // North
        map = roll_stones(&map);
        map = rotate_map(&map);
        // East
        map = roll_stones(&map);
        map = rotate_map(&map);
        // South
        map = roll_stones(&map);
        map = rotate_map(&map);
        // West
        map = roll_stones(&map);
        map = rotate_map(&map);
    }

    let cycle_size = cycle_end_index - cycle_start_index;
    let remainder = ROUNDS % cycle_size;

    for (map, index) in prev_maps.into_iter() {
        if index % cycle_size == remainder {
            return calc_load(&map).to_string();
        }
    }

    panic!("Failed to find answer to part 2");
}

fn roll_stones(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_map = map.clone();

    for y in 0..map.len() {
        let row = &map[y];
        for x in 0..row.len() {
            let col = &row[x];
            match col {
                b'O' => {
                    // Roll northwards.
                    let mut new_y = y;
                    for roll_y in (0..y).rev() {
                        if new_map[roll_y][x] != b'.' {
                            break;
                        }

                        new_y = roll_y;
                    }

                    new_map[y][x] = b'.';
                    new_map[new_y][x] = b'O';
                }
                b'.' => {
                    new_map[y][x] = b'.';
                }
                b'#' => {
                    new_map[y][x] = b'#';
                }
                _ => panic!("Invalid char {col}"),
            }
        }
    }

    new_map
}

// Rotates the map 90 deg clockwise.
fn rotate_map(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut rotated = map.clone();

    let height = map.len();
    for y in 0..height {
        let width = map[y].len();
        for x in 0..width {
            rotated[y][x] = map[width - 1 - x][y];
        }
    }

    rotated
}

fn calc_load(map: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == b'O' {
                sum += map.len() - y;
            }
        }
    }

    sum
}

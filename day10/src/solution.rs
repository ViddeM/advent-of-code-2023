use std::{
    collections::{HashMap, HashSet},
    ops::Div,
};

pub fn parse<'a>(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn get_start_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile == &'S' {
                return (x, y);
            }
        }
    }

    panic!("ASD")
}

#[derive(Debug, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn get_next_dir_at_start(map: &Vec<Vec<char>>, start_x: &usize, start_y: &usize) -> Dir {
    if start_y > &0 {
        match map[start_y - 1][*start_x] {
            '|' => return Dir::Up,
            '7' => return Dir::Up,
            'F' => return Dir::Up,
            _ => {}
        }
    }

    if let Some(c) = map[*start_y].get(start_x + 1) {
        match c {
            '-' => return Dir::Right,
            'J' => return Dir::Right,
            '7' => return Dir::Right,
            _ => {}
        }
    }

    if let Some(row) = map.get(start_y + 1) {
        match row[*start_x] {
            '|' => return Dir::Down,
            'L' => return Dir::Down,
            'J' => return Dir::Down,
            _ => {}
        }
    }

    if start_x > &0 {
        match map[*start_y][start_x - 1] {
            '-' => return Dir::Left,
            'L' => return Dir::Left,
            'F' => return Dir::Left,
            _ => {}
        }
    }

    panic!("no available dir from start! Expected 2");
}

pub fn solve_part_one<'a>(input: Vec<Vec<char>>) -> String {
    let (start_x, start_y) = get_start_pos(&input);

    let mut curr_x = start_x;
    let mut curr_y = start_y;

    let mut came_from = Dir::Up; // Randomly selected;
    let mut next_dir = get_next_dir_at_start(&input, &start_x, &start_y);

    let mut steps = 0;

    loop {
        match next_dir {
            Dir::Up => {
                came_from = Dir::Down;
                curr_y -= 1;
            }
            Dir::Right => {
                came_from = Dir::Left;
                curr_x += 1;
            }
            Dir::Down => {
                came_from = Dir::Up;
                curr_y += 1;
            }
            Dir::Left => {
                came_from = Dir::Right;
                curr_x -= 1;
            }
        }

        match (input[curr_y][curr_x], came_from) {
            ('|', Dir::Up) => next_dir = Dir::Down,
            ('|', Dir::Down) => next_dir = Dir::Up,
            ('-', Dir::Left) => next_dir = Dir::Right,
            ('-', Dir::Right) => next_dir = Dir::Left,
            ('L', Dir::Up) => next_dir = Dir::Right,
            ('L', Dir::Right) => next_dir = Dir::Up,
            ('J', Dir::Up) => next_dir = Dir::Left,
            ('J', Dir::Left) => next_dir = Dir::Up,
            ('7', Dir::Left) => next_dir = Dir::Down,
            ('7', Dir::Down) => next_dir = Dir::Left,
            ('F', Dir::Right) => next_dir = Dir::Down,
            ('F', Dir::Down) => next_dir = Dir::Right,
            ('S', _) => break,
            ('.', _) => panic!("Got to ground tile!"),

            (c, d) => {
                panic!("Invalid combination '{d:?}' '{c}'")
            }
        }

        steps += 1;
    }

    let half = steps as f64 / 2.0;
    half.ceil().to_string()
}

fn get_start_char(map: &Vec<Vec<char>>, start_x: &usize, start_y: &usize) -> char {
    let up = if start_y > &0 {
        match map[start_y - 1][*start_x] {
            '|' => true,
            '7' => true,
            'F' => true,
            _ => false,
        }
    } else {
        false
    };

    let right = if let Some(c) = map[*start_y].get(start_x + 1) {
        match c {
            '-' => true,
            'J' => true,
            '7' => true,
            _ => false,
        }
    } else {
        false
    };

    let down = if let Some(row) = map.get(start_y + 1) {
        match row[*start_x] {
            '|' => true,
            'L' => true,
            'J' => true,
            _ => false,
        }
    } else {
        false
    };

    let left = if start_x > &0 {
        match map[*start_y][start_x - 1] {
            '-' => true,
            'L' => true,
            'F' => true,
            _ => false,
        }
    } else {
        false
    };

    match (up, right, down, left) {
        (true, true, false, false) => 'L',
        (true, false, true, false) => '|',
        (true, false, false, true) => 'J',
        (false, true, true, false) => 'F',
        (false, true, false, true) => '-',
        (false, false, true, true) => '7',
        _ => panic!("Invalid start tile!"),
    }
}

pub fn solve_part_two<'a>(input: Vec<Vec<char>>) -> String {
    let mut map = input;

    let (start_x, start_y) = get_start_pos(&map);
    map[start_y][start_x] = get_start_char(&map, &start_x, &start_y);

    let mut part_of_loop: HashSet<(usize, usize)> = HashSet::new();

    let mut curr_x = start_x;
    let mut curr_y = start_y;
    let mut came_from = Dir::Up; // Randomly selected;
    let mut next_dir = get_next_dir_at_start(&map, &start_x, &start_y);
    loop {
        part_of_loop.insert((curr_x, curr_y));
        match next_dir {
            Dir::Up => {
                came_from = Dir::Down;
                curr_y -= 1;
            }
            Dir::Right => {
                came_from = Dir::Left;
                curr_x += 1;
            }
            Dir::Down => {
                came_from = Dir::Up;
                curr_y += 1;
            }
            Dir::Left => {
                came_from = Dir::Right;
                curr_x -= 1;
            }
        }

        if curr_x == start_x && curr_y == start_y {
            break;
        }

        match (map[curr_y][curr_x], came_from) {
            ('|', Dir::Up) => next_dir = Dir::Down,
            ('|', Dir::Down) => next_dir = Dir::Up,
            ('-', Dir::Left) => next_dir = Dir::Right,
            ('-', Dir::Right) => next_dir = Dir::Left,
            ('L', Dir::Up) => next_dir = Dir::Right,
            ('L', Dir::Right) => next_dir = Dir::Up,
            ('J', Dir::Up) => next_dir = Dir::Left,
            ('J', Dir::Left) => next_dir = Dir::Up,
            ('7', Dir::Left) => next_dir = Dir::Down,
            ('7', Dir::Down) => next_dir = Dir::Left,
            ('F', Dir::Right) => next_dir = Dir::Down,
            ('F', Dir::Down) => next_dir = Dir::Right,
            ('S', _) => panic!("S tile should've been removed!"),
            ('.', _) => panic!("Got to ground tile!"),

            (c, d) => {
                panic!("Invalid combination '{d:?}' '{c}'")
            }
        };
    }

    let vertical_chars = vec!['|', 'J', 'L'];
    let mut count = 0;
    for (y, row) in map.into_iter().enumerate() {
        let mut inside = false;
        for (x, c) in row.into_iter().enumerate() {
            if part_of_loop.contains(&(x, y)) {
                if vertical_chars.contains(&c) {
                    inside = !inside;
                }
            } else if inside {
                count += 1;
            }
        }
    }

    count.to_string()
}

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn get_next(&self, x: usize, y: usize, map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        match self {
            Dir::Up => {
                if y == 0 {
                    return None;
                }
                return Some((x, y - 1));
            }
            Dir::Right => {
                if x == map[0].len() - 1 {
                    return None;
                }
                return Some((x + 1, y));
            }
            Dir::Down => {
                if y == map.len() - 1 {
                    return None;
                }
                return Some((x, y + 1));
            }
            Dir::Left => {
                if x == 0 {
                    return None;
                }
                return Some((x - 1, y));
            }
        }
    }

    // Handle '/'
    fn tilt_right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Down,
        }
    }

    // Handle '\'
    fn tilt_left(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Up,
        }
    }

    // Handle '-'
    fn handle_horizontal_split(&self) -> Option<(Dir, Dir)> {
        match self {
            Dir::Up => Some((Dir::Left, Dir::Right)),
            Dir::Right => None,
            Dir::Down => Some((Dir::Left, Dir::Right)),
            Dir::Left => None,
        }
    }

    // Handle '|'
    fn handle_vertical_split(&self) -> Option<(Dir, Dir)> {
        match self {
            Dir::Up => None,
            Dir::Right => Some((Dir::Up, Dir::Down)),
            Dir::Down => None,
            Dir::Left => Some((Dir::Up, Dir::Down)),
        }
    }
}

fn run_for_entrance(start: (usize, usize, Dir), map: &Vec<Vec<char>>) -> usize {
    let mut to_check: Vec<(usize, usize, Dir)> = Vec::new();
    let mut splits: HashSet<(usize, usize)> = HashSet::new();

    let mut energized: HashSet<(usize, usize)> = HashSet::new();

    to_check.push(start);

    while to_check.len() > 0 {
        let (mut x, mut y, mut going_in) = to_check.swap_remove(0);
        'beam: loop {
            match map[y][x] {
                '.' => {}
                '/' => {
                    going_in = going_in.tilt_right();
                }
                '\\' => {
                    going_in = going_in.tilt_left();
                }
                '|' => {
                    if let Some((dir_a, dir_b)) = going_in.handle_vertical_split() {
                        if splits.contains(&(x, y)) {
                            // In an inf loop.
                            break 'beam;
                        }
                        splits.insert((x, y));

                        going_in = dir_a;
                        if let Some((new_x, new_y)) = dir_b.get_next(x, y, map) {
                            to_check.push((new_x, new_y, dir_b));
                        }
                    }
                }
                '-' => {
                    if let Some((dir_a, dir_b)) = going_in.handle_horizontal_split() {
                        if splits.contains(&(x, y)) {
                            // In an inf loop.
                            break 'beam;
                        }
                        splits.insert((x, y));

                        going_in = dir_a;
                        if let Some((new_x, new_y)) = dir_b.get_next(x, y, map) {
                            to_check.push((new_x, new_y, dir_b));
                        }
                    }
                }
                v => {
                    panic!("Invalid input '{v:?}'");
                }
            }

            energized.insert((x, y));
            if let Some((new_x, new_y)) = going_in.get_next(x, y, map) {
                x = new_x;
                y = new_y;
            } else {
                break 'beam;
            }
        }
    }

    energized.len()
}

pub fn parse<'a>(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn solve_part_one<'a>(input: Vec<Vec<char>>) -> String {
    run_for_entrance((0, 0, Dir::Right), &input).to_string()
}

pub fn solve_part_two<'a>(input: Vec<Vec<char>>) -> String {
    let mut highest = 0;

    let height = input.len();
    let width = input[0].len();

    for y in 0..height {
        let num = run_for_entrance((0, y, Dir::Right), &input);
        if num > highest {
            highest = num;
        }

        let num = run_for_entrance((width - 1, y, Dir::Left), &input);
        if num > highest {
            highest = num;
        }
    }

    for x in 0..width {
        let num = run_for_entrance((x, 0, Dir::Down), &input);
        if num > highest {
            highest = num;
        }

        let num = run_for_entrance((x, height - 1, Dir::Up), &input);
        if num > highest {
            highest = num;
        }
    }

    highest.to_string()
}

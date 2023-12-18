use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

pub struct Instruction {
    dir: Dir,
    steps: i64,
    hex_code: String,
}

pub fn parse<'a>(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let (dir, res) = l.split_once(" ").unwrap();
            let (steps, hex_code) = res.split_once(" ").unwrap();

            Instruction {
                dir: match dir {
                    "R" => Dir::Right,
                    "U" => Dir::Up,
                    "D" => Dir::Down,
                    "L" => Dir::Left,
                    _ => panic!("Invalid dir {dir}"),
                },
                steps: steps.parse().unwrap(),
                hex_code: hex_code
                    .strip_prefix("(#")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .to_string(),
            }
        })
        .collect()
}

fn print_map(map: &Vec<Vec<bool>>) {
    let m = map
        .iter()
        .map(|l| {
            l.iter()
                .enumerate()
                .filter(|(index, _)| index < &200)
                .map(|(_, a)| a)
                .map(|c| if *c { "#" } else { "." }.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("M:\n{m}");
}

fn determine_edges(instructions: &Vec<Instruction>) -> (i64, i64, i64, i64) {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    let mut curr_x = 0;
    let mut curr_y = 0;
    for ins in instructions.iter() {
        match ins.dir {
            Dir::Up => {
                curr_y -= ins.steps;
                if curr_y < min_y {
                    min_y = curr_y;
                }
            }
            Dir::Right => {
                curr_x += ins.steps;
                if curr_x > max_x {
                    max_x = curr_x;
                }
            }
            Dir::Down => {
                curr_y += ins.steps;
                if curr_y > max_y {
                    max_y = curr_y;
                }
            }
            Dir::Left => {
                curr_x -= ins.steps;
                if curr_x < min_x {
                    min_x = curr_x;
                }
            }
        }
    }

    (min_x, max_x, min_y, max_y)
}

fn create_map(
    instructions: &Vec<Instruction>,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
) -> Vec<Vec<bool>> {
    let mut curr_x = start_x;
    let mut curr_y = start_y;

    let mut map = vec![vec![false; width]; height];
    map[curr_y][curr_x] = true;
    for ins in instructions.iter() {
        match ins.dir {
            Dir::Up => {
                let new_y = curr_y - ins.steps as usize;
                for y in new_y..=curr_y {
                    map[y][curr_x] = true;
                }
                curr_y = new_y;
            }
            Dir::Right => {
                let new_x = curr_x + ins.steps as usize;
                for x in curr_x..=new_x {
                    map[curr_y][x] = true;
                }
                curr_x = new_x;
            }
            Dir::Down => {
                let new_y = curr_y + ins.steps as usize;
                for y in curr_y..=new_y {
                    map[y][curr_x] = true;
                }
                curr_y = new_y;
            }
            Dir::Left => {
                let new_x = curr_x - ins.steps as usize;
                for x in new_x..=curr_x {
                    map[curr_y][x] = true;
                }
                curr_x = new_x;
            }
        }
    }
    map
}

fn flood_fill(
    map: &Vec<Vec<bool>>,
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,
) -> HashSet<(usize, usize)> {
    let mut checked = HashSet::new();
    let mut to_check = HashSet::new();

    to_check.insert((start_x + 1, start_y + 1));

    while !to_check.is_empty() {
        to_check = flood_fill_inner(&to_check, &mut checked, map, width, height);
    }

    checked
}

fn flood_fill_inner(
    to_check: &HashSet<(usize, usize)>,
    checked: &mut HashSet<(usize, usize)>,
    map: &Vec<Vec<bool>>,
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    let mut new_to_check = vec![];
    for (x, y) in to_check.iter() {
        let val = map[*y][*x];
        if !val {
            checked.insert((*x, *y));

            if y > &0 {
                new_to_check.push((*x, y - 1));
            }
            if *y < height - 1 {
                new_to_check.push((*x, y + 1));
            }
            if *x > 0 {
                new_to_check.push((x - 1, *y));
            }
            if *x < width - 1 {
                new_to_check.push((x + 1, *y));
            }
        }
    }

    new_to_check
        .into_iter()
        .filter(|(x, y)| !checked.contains(&(*x, *y)))
        .collect::<HashSet<(usize, usize)>>()
}

pub fn solve_part_one<'a>(input: Vec<Instruction>) -> String {
    let (min_x, max_x, min_y, max_y) = determine_edges(&input);

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    let mut map = create_map(
        &input,
        min_x.abs() as usize,
        min_y.abs() as usize,
        width,
        height,
    );

    println!("Ended up with map of width {width} and height {height}");

    let inner = flood_fill(
        &map,
        width,
        height,
        min_x.abs() as usize,
        min_y.abs() as usize,
    );

    for (x, y) in inner.into_iter() {
        map[y][x] = true;
    }

    map.into_iter()
        .map(|row| row.into_iter().filter(|c| *c).count())
        .sum::<usize>()
        .to_string()
}

pub fn solve_part_two<'a>(input: Vec<Instruction>) -> String {
    let new_ins = input
        .into_iter()
        .map(|ins| {
            let steps = i64::from_str_radix(&ins.hex_code[0..5], 16).unwrap();
            let dir = match ins.hex_code.chars().nth(5).unwrap() {
                '0' => Dir::Right,
                '1' => Dir::Down,
                '2' => Dir::Left,
                '3' => Dir::Up,
                _ => panic!("Invalid char"),
            };

            (dir, steps)
        })
        .collect::<Vec<(Dir, i64)>>();

    let mut start_x = 0;
    let mut start_y = 0;
    let mut perim = vec![];
    let mut perim_length = 0;

    for (dir, steps) in new_ins.iter() {
        let (end_x, end_y) = match dir {
            Dir::Up => (start_x, start_y - steps),
            Dir::Right => (start_x + steps, start_y),
            Dir::Down => (start_x, start_y + steps),
            Dir::Left => (start_x - steps, start_y),
        };

        perim_length += steps;
        perim.push((end_x, end_y));
        start_x = end_x;
        start_y = end_y;
    }

    (perim_length / 2
        + perim
            .windows(2)
            .map(|l| {
                let (a_x, a_y) = l[0];
                let (b_x, b_y) = l[1];
                (a_y + b_y) * (a_x - b_x)
            })
            .sum::<i64>()
            / 2
        + 1)
    .to_string()
}

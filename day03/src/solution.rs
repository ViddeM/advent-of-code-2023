use std::collections::{HashMap, HashSet};

pub struct Map {
    width: usize,
    height: usize,
    numbers: HashMap<(usize, usize), usize>,
    symbols: HashSet<(usize, usize)>,
    gears: HashSet<(usize, usize)>,
}

impl Map {
    fn get_all_symbol_neighbours(&self) -> HashSet<(usize, usize)> {
        let mut all_neighs = HashSet::new();

        for (x, y) in self.symbols.iter() {
            let start_x = if x == &0 { 0 } else { x - 1 };
            let start_y = if y == &0 { 0 } else { y - 1 };

            let end_x = if x == &(self.width - 1) {
                self.width - 1
            } else {
                x + 1
            };
            let end_y = if y == &(self.height - 1) {
                self.height - 1
            } else {
                y + 1
            };

            for in_y in start_y..=end_y {
                for in_x in start_x..=end_x {
                    all_neighs.insert((in_x, in_y));
                }
            }
        }

        all_neighs
    }

    fn get_gear_neighbours(&self) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
        let mut all_neighs = HashMap::new();

        for (x, y) in self.gears.iter() {
            let start_x = if x == &0 { 0 } else { x - 1 };
            let start_y = if y == &0 { 0 } else { y - 1 };

            let end_x = if x == &(self.width - 1) {
                self.width - 1
            } else {
                x + 1
            };
            let end_y = if y == &(self.height - 1) {
                self.height - 1
            } else {
                y + 1
            };

            for in_y in start_y..=end_y {
                for in_x in start_x..=end_x {
                    let mut set = all_neighs.remove(&(in_x, in_y)).unwrap_or(HashSet::new());
                    set.insert((*x, *y));
                    all_neighs.insert((in_x, in_y), set);
                }
            }
        }

        all_neighs
    }
}

pub fn parse<'a>(input: &str) -> Map {
    let mut numbers = HashMap::new();
    let mut symbols = HashSet::new();
    let mut gears = HashSet::new();

    let mut width = 0;
    let height = input.lines().count();

    for (y, line) in input.trim().lines().enumerate() {
        let mut num = String::new();
        let mut num_start = None;
        width = line.len();

        for (x, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                num.push(char);

                if num_start.is_none() {
                    num_start = Some((x, y));
                }
            } else {
                if num.len() > 0 {
                    numbers.insert(
                        num_start.expect("No num start"),
                        num.parse().expect("Invalid number"),
                    );
                    num = String::new();
                    num_start = None;
                }

                if char != '.' {
                    // is a symbol!
                    symbols.insert((x, y));

                    if char == '*' {
                        gears.insert((x, y));
                    }
                }
            }
        }

        if num.len() > 0 {
            numbers.insert(
                num_start.expect("No num start"),
                num.parse().expect("Invalid number"),
            );
            num = String::new();
            num_start = None;
        }
    }

    Map {
        numbers,
        symbols,
        width,
        height,
        gears,
    }
}

pub fn solve_part_one<'a>(input: Map) -> String {
    let all_neighs = input.get_all_symbol_neighbours();

    input
        .numbers
        .iter()
        .filter(|((x, y), num)| {
            let start = x.clone();
            let end = x + num.to_string().len();
            for in_x in start..end {
                if all_neighs.contains(&(in_x, *y)) {
                    return true;
                }
            }

            return false;
        })
        .map(|(_, num)| num)
        .sum::<usize>()
        .to_string()
}

pub fn solve_part_two<'a>(input: Map) -> String {
    let mut gear_to_num_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    let gear_positions: HashMap<(usize, usize), HashSet<(usize, usize)>> =
        input.get_gear_neighbours();

    for ((x, y), num) in input.numbers.iter() {
        let mut neighbouring_gears = vec![];

        let start = x.clone();
        let end = x + num.to_string().len();
        for in_x in start..end {
            if let Some(gear_positions) = gear_positions.get(&(in_x, *y)) {
                for gear_pos in gear_positions.iter() {
                    if neighbouring_gears.contains(gear_pos) == false {
                        neighbouring_gears.push(gear_pos.clone());

                        if let Some(curr_nums) = gear_to_num_map.get_mut(gear_pos) {
                            curr_nums.push(num.clone());
                        } else {
                            gear_to_num_map.insert(gear_pos.clone(), vec![num.clone()]);
                        }
                    }
                }
            }
        }
    }

    gear_to_num_map
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| {
            let a = numbers[0];
            let b = numbers[1];
            a * b
        })
        .sum::<usize>()
        .to_string()
}

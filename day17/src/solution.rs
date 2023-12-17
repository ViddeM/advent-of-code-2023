use std::collections::HashMap;

use priority_queue::PriorityQueue;

pub fn parse<'a>(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.bytes().map(|c| c - 48).collect())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        }
    }

    fn left_right(&self) -> Vec<Dir> {
        let dirs = vec![Dir::Up, Dir::Right, Dir::Down, Dir::Left];
        dirs.into_iter()
            .filter(|d| self != d)
            .filter(|d| &self.opposite() != d)
            .collect()
    }

    fn delta_x(&self) -> i32 {
        match self {
            Dir::Up => 0,
            Dir::Right => 1,
            Dir::Down => 0,
            Dir::Left => -1,
        }
    }

    fn delta_y(&self) -> i32 {
        match self {
            Dir::Up => -1,
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 0,
        }
    }
}

fn path_find(map: &Vec<Vec<u8>>, min_steps: i32, max_steps: i32) -> usize {
    let mut to_check: PriorityQueue<((usize, usize), Dir), i64> = PriorityQueue::new();
    to_check.push(((0, 0), Dir::Right), 0);
    to_check.push(((0, 0), Dir::Down), 0);

    let mut dists: HashMap<((usize, usize), Dir), usize> = HashMap::new();

    let height = map.len() - 1;
    let width = map[0].len() - 1;

    while let Some((((x, y), dir), cost)) = to_check.pop() {
        // Goal check
        if y == height && x == width {
            return (-cost) as usize;
        }

        // Prevent loops
        if dists
            .get(&((x, y), dir.clone()))
            .is_some_and(|&alt_cost| (-cost) as usize > alt_cost)
        {
            continue;
        }

        // Check neighbours
        for neighbour in dir.left_right() {
            let mut new_cost = (-cost) as usize;
            for step in 1..=max_steps {
                let new_x = x as i32 + neighbour.delta_x() * step;
                let new_y = y as i32 + neighbour.delta_y() * step;
                if new_x < 0 || new_y < 0 {
                    continue;
                }

                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if new_x > width || new_y > height {
                    continue;
                }

                new_cost += map[new_y][new_x] as usize;

                let entry = ((new_x, new_y), neighbour.clone());
                if &new_cost < dists.get(&entry).unwrap_or(&usize::MAX) && step >= min_steps {
                    dists.insert(entry.clone(), new_cost);
                    to_check.push(entry, -(new_cost as i64));
                }
            }
        }
    }
    panic!("Found no path :(");
}

pub fn solve_part_one<'a>(input: Vec<Vec<u8>>) -> String {
    path_find(&input, 1, 3).to_string()
}

pub fn solve_part_two<'a>(input: Vec<Vec<u8>>) -> String {
    path_find(&input, 4, 10).to_string()
}

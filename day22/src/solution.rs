use std::collections::{HashMap, HashSet};

pub struct Inp {
    bricks: HashMap<usize, ((usize, usize, usize), (usize, usize, usize))>,
    width: usize,  // x
    depth: usize,  // y
    height: usize, // z
}

pub fn parse<'a>(input: &'a str) -> Inp {
    let mut width = 0;
    let mut depth = 0;
    let mut height = 0;

    let bricks = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (start, end) = l.split_once("~").unwrap();
            let s = start
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>();
            let e = end
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>();

            if s[0] > width {
                width = s[0];
            }
            if e[0] > width {
                width = e[0];
            }
            if s[1] > depth {
                depth = s[1];
            }
            if e[1] > depth {
                depth = e[1];
            }
            if s[2] > height {
                height = s[2];
            }
            if e[2] > height {
                height = e[2];
            }

            (i + 1, ((s[0], s[1], s[2]), (e[0], e[1], e[2])))
        })
        .collect();

    Inp {
        bricks,
        width,
        depth,
        height,
    }
}

fn print_map_from_x(map: &Vec<Vec<Vec<usize>>>) {
    for (z, level) in map.iter().enumerate().rev() {
        for row in level.iter() {
            let mut c = '.';
            for id in row.iter() {
                if id != &0 {
                    c = format!("{id}")
                        .chars()
                        .collect::<Vec<char>>()
                        .first()
                        .unwrap()
                        .clone();
                }
            }
            print!("{}", c);
        }
        println!(" - {z}");
    }
}

fn print_map_from_y(map: &Vec<Vec<Vec<usize>>>) {
    for (z, level) in map.iter().enumerate().rev() {
        for row in level.iter() {
            let mut c = '.';
            for id in row.iter() {
                if id != &0 {
                    c = format!("{id}")
                        .chars()
                        .collect::<Vec<char>>()
                        .first()
                        .unwrap()
                        .clone();
                }
            }
            print!("{}", c);
        }
        println!(" - {z}");
    }
}

fn find_brick_deps(input: &Inp) -> HashMap<usize, HashSet<usize>> {
    // Indexed [z][y][x]
    let mut finished_map = vec![vec![vec![0; input.width + 1]; input.depth + 1]; input.height + 1];

    let mut bricks_sorting = input
        .bricks
        .iter()
        .map(|(id, ((_, _, s_z), _))| (id, s_z))
        .collect::<Vec<(&usize, &usize)>>();
    bricks_sorting.sort_by(|(_, a), (_, b)| a.cmp(b));

    let mut brick_z_offset = HashMap::new();
    for (id, low_z) in bricks_sorting.iter() {
        let mut lower_by = 0;
        let brick = input.bricks.get(id).unwrap();
        'test_level: for test_z in (1..**low_z).rev() {
            // Check all the bricks tiles at this new z
            for x in brick.0 .0..=brick.1 .0 {
                for y in brick.0 .1..=brick.1 .1 {
                    if finished_map[test_z][y][x] != 0 {
                        break 'test_level;
                    }
                }
            }
            lower_by += 1;
        }
        brick_z_offset.insert(**id, lower_by);

        // Now actually place the brick at the lower position
        for x in brick.0 .0..=brick.1 .0 {
            for y in brick.0 .1..=brick.1 .1 {
                for z in brick.0 .2..=brick.1 .2 {
                    let act_z = z - lower_by;
                    if finished_map[act_z][y][x] != 0 {
                        panic!("THERE IS ALREADY SOMETHING AT {x} {y} {act_z} for brick {id}");
                    }

                    finished_map[act_z][y][x] = **id;
                }
            }
        }
    }

    // Brick ID -> Set<BrickId>
    let mut brick_deps: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (id, _) in bricks_sorting.iter() {
        let brick = input.bricks.get(id).unwrap();
        let z_offset = brick_z_offset.get(id).unwrap();

        let z = brick.0 .2 - z_offset - 1;
        let mut deps = HashSet::new();
        if z >= 1 {
            for x in brick.0 .0..=brick.1 .0 {
                for y in brick.0 .1..=brick.1 .1 {
                    if finished_map[z][y][x] != 0 {
                        deps.insert(finished_map[z][y][x]);
                    }
                }
            }
        }
        brick_deps.insert(**id, deps);
    }

    brick_deps
}

fn find_brick_inv_deps(
    brick_deps: &HashMap<usize, HashSet<usize>>,
) -> HashMap<usize, HashSet<usize>> {
    let mut brick_inv_deps = HashMap::new();
    for (id, _) in brick_deps.iter() {
        let mut inv_deps = HashSet::new();
        for (id2, deps) in brick_deps.iter() {
            if id != id2 && deps.contains(id) {
                inv_deps.insert(*id2);
            }
        }
        brick_inv_deps.insert(*id, inv_deps);
    }
    brick_inv_deps
}

pub fn solve_part_one<'a>(input: Inp) -> String {
    let brick_deps = find_brick_deps(&input);
    let brick_inv_deps = find_brick_inv_deps(&brick_deps);

    brick_inv_deps
        .values()
        .filter(|deps| {
            !deps
                .iter()
                .any(|dep| brick_deps.get(dep).unwrap().len() == 1)
        })
        .count()
        .to_string()
}

pub fn solve_part_two<'a>(input: Inp) -> String {
    let brick_deps = find_brick_deps(&input);
    let brick_inv_deps = find_brick_inv_deps(&brick_deps);

    let mut sum = 0;
    for (brick, deps) in brick_inv_deps.iter() {
        let mut disintegrated = HashSet::new();
        disintegrated.insert(brick);

        let mut deps_to_check = vec![];
        for dep in deps.iter() {
            deps_to_check.push(dep);
        }

        while let Some(dep) = deps_to_check.pop() {
            // First check if the brick will be disintegrated
            // Then if it is, check its children
            if brick_deps
                .get(dep)
                .unwrap()
                .iter()
                .all(|d| disintegrated.contains(d))
            {
                // The brick is disintegrated
                disintegrated.insert(dep);
                for child_dep in brick_inv_deps.get(dep).unwrap() {
                    deps_to_check.push(child_dep);
                }
            }
        }

        // Remove ourselves
        disintegrated.remove(brick);
        sum += disintegrated.len();
    }

    sum.to_string()
}

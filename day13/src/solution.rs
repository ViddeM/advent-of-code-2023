#[derive(Debug, Clone)]
pub struct Map {
    map: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

pub fn parse<'a>(input: &str) -> Vec<Map> {
    input
        .split("\n\n")
        .map(|map| {
            let map: Vec<Vec<u8>> = map.lines().map(|l| l.bytes().collect()).collect();
            let height = map.len();
            let width = map[0].len();

            Map { map, width, height }
        })
        .collect()
}

fn find_horizontal_line(map: &Map, allowed_diff: usize) -> Option<usize> {
    for y in 0..map.height - 1 {
        let rows_to_check = (map.height - (y + 1)).min(y + 1);

        if (0..rows_to_check)
            .map(|test_y| {
                let r = &map.map[y - test_y];
                let other_r = &map.map[y + 1 + test_y];
                (0..map.width)
                    .map(|x| r[x] == other_r[x])
                    .map(|b| if b { 0 } else { 1 })
                    .sum::<usize>()
            })
            .sum::<usize>()
            == allowed_diff
        {
            return Some(y + 1);
        }
    }

    None
}

fn find_vertical_line(map: &Map, allowed_diff: usize) -> Option<usize> {
    for x in 0..map.width - 1 {
        let cols_to_check = (map.width - (x + 1)).min(x + 1);

        if (0..cols_to_check)
            .map(|test_x| {
                (0..map.height)
                    .map(|y| {
                        let row = &map.map[y];
                        row[x - test_x] == row[x + 1 + test_x]
                    })
                    .map(|b| if b { 0 } else { 1 })
                    .sum::<usize>()
            })
            .sum::<usize>()
            == allowed_diff
        {
            return Some(x + 1);
        }
    }

    None
}

pub fn solve_part_one<'a>(input: Vec<Map>) -> String {
    input
        .into_iter()
        .map(|map| {
            if let Some(line) = find_horizontal_line(&map, 0) {
                line * 100
            } else {
                let line = find_vertical_line(&map, 0)
                    .expect("No horizontal line existed so a vertical line should!");
                line
            }
        })
        .sum::<usize>()
        .to_string()
}

pub fn solve_part_two<'a>(input: Vec<Map>) -> String {
    input
        .into_iter()
        .map(|map| {
            if let Some(line) = find_horizontal_line(&map, 1) {
                line * 100
            } else {
                let line = find_vertical_line(&map, 1)
                    .expect("No horizontal line existed so a vertical line should!");
                line
            }
        })
        .sum::<usize>()
        .to_string()
}

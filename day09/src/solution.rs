pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Vec<i64>> + 'a {
    input
        .lines()
        .map(|l| l.split(" ").map(|n| n.parse().unwrap()).collect())
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = Vec<i64>>) -> String {
    let mut sum = 0;

    for row in input {
        let mut lines = vec![row.clone()];
        loop {
            let mut next = vec![];
            let line = lines.last().unwrap();
            for (i, c) in line.iter().enumerate() {
                if i == line.len() - 1 {
                    break;
                }

                let n = line[i + 1];
                next.push(n - c)
            }

            if next.iter().all(|n| n == &0) {
                lines.push(next);
                break;
            }

            lines.push(next);
        }

        lines.last_mut().unwrap().push(0);

        for line_index in (0..lines.len() - 1).rev() {
            let new_num = {
                let curr = lines[line_index].last().unwrap();
                let prev = lines[line_index + 1].last().unwrap();

                curr + prev
            };

            lines.get_mut(line_index).unwrap().push(new_num);
        }

        sum += lines.first().unwrap().last().unwrap();
    }
    sum.to_string()
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = Vec<i64>>) -> String {
    let mut sum = 0;

    for row in input {
        let mut lines = vec![row.clone()];
        loop {
            let mut next = vec![];
            let line = lines.last().unwrap();
            for (i, c) in line.iter().enumerate() {
                if i == line.len() - 1 {
                    break;
                }

                let n = line[i + 1];
                next.push(n - c)
            }

            if next.iter().all(|n| n == &0) {
                lines.push(next);
                break;
            }

            lines.push(next);
        }

        lines.last_mut().unwrap().insert(0, 0);

        for line_index in (0..lines.len() - 1).rev() {
            let new_num = {
                let curr = lines[line_index].first().unwrap();
                let prev = lines[line_index + 1].first().unwrap();

                curr - prev
            };

            lines.get_mut(line_index).unwrap().insert(0, new_num);
        }

        sum += lines.first().unwrap().first().unwrap();
    }
    sum.to_string()
}

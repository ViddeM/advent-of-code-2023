use std::collections::{HashMap, HashSet};

pub fn parse<'a>(input: &'a str) -> Vec<(&str, &str)> {
    input
        .lines()
        .flat_map(|l| {
            let (a, bs) = l.split_once(": ").unwrap();
            bs.split(" ").map(move |b| (a, b))
        })
        .collect()
}

fn to_edges<'a>(pairs: &Vec<(&'a str, &'a str)>) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut vertices = HashSet::new();
    for (a, b) in pairs.iter() {
        if !vertices.contains(a) {
            vertices.insert(a);
        }

        if !vertices.contains(b) {
            vertices.insert(b);
        }
    }

    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for vert in vertices.iter() {
        edges.insert(vert, HashSet::new());
    }

    for (a, b) in pairs.iter() {
        edges.get_mut(a).unwrap().insert(b);
        edges.get_mut(b).unwrap().insert(a);
    }
    edges
}

fn find_frequencies<'a>(
    edges: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashMap<(&'a str, &'a str), usize> {
    let mut frequencies = HashMap::new();
    for e in edges.keys() {
        let mut to_check = vec![];
        to_check.insert(0, *e);

        let mut visited = HashSet::new();
        visited.insert(*e);

        while let Some(pos) = to_check.pop() {
            for &next in &edges[pos] {
                if visited.insert(next) {
                    let key = if pos < next { (pos, next) } else { (next, pos) };

                    let freq = frequencies.entry(key).or_insert(0);
                    *freq += 1;

                    to_check.insert(0, next);
                }
            }
        }
    }
    frequencies
}

fn find_partition_size(edges: &HashMap<&str, HashSet<&str>>, divided: &Vec<(&str, &str)>) -> usize {
    let node = *edges.keys().next().unwrap();
    let mut size = 1;

    let mut to_check = vec![];
    to_check.insert(0, node);

    let mut visited = HashSet::new();
    visited.insert(node);

    while let Some(pos) = to_check.pop() {
        for &next in &edges[pos] {
            let key = if pos < next { (pos, next) } else { (next, pos) };

            if divided.contains(&key) {
                continue;
            }

            if visited.insert(next) {
                size += 1;
                to_check.insert(0, next);
            }
        }
    }

    size
}

pub fn solve_part_one<'a>(input: Vec<(&str, &str)>) -> String {
    let edges = to_edges(&input);
    let frequencies = find_frequencies(&edges);

    let mut sorted = frequencies
        .iter()
        .map(|(a, b)| (*a, *b))
        .collect::<Vec<((&str, &str), usize)>>();
    sorted.sort_by_key(|(_, freq)| freq.clone());
    sorted.reverse();

    let divided = sorted
        .iter()
        .take(3)
        .map(|(e, _)| *e)
        .collect::<Vec<(&str, &str)>>();

    let size = find_partition_size(&edges, &divided);

    (size * (edges.len() - size)).to_string()
}

pub fn solve_part_two<'a>(input: Vec<(&str, &str)>) -> String {
    panic!("Part two is not available!")
}

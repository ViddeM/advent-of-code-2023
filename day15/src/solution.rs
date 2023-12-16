use std::collections::HashMap;

pub fn parse<'a>(input: &str) -> Vec<Vec<u8>> {
    input
        .replace("\n", "")
        .split(",")
        .map(|seq| seq.bytes().collect())
        .collect()
}

fn hash(bytes: &[u8]) -> usize {
    let mut current_value: usize = 0;
    for b in bytes.into_iter() {
        current_value += *b as usize;
        current_value *= 17;
        current_value = current_value % 256;
    }
    current_value
}

pub fn solve_part_one<'a>(input: Vec<Vec<u8>>) -> String {
    let mut sum: usize = 0;

    for seq in input.into_iter() {
        sum += hash(seq.as_slice());
    }

    sum.to_string()
}

pub fn solve_part_two<'a>(input: Vec<Vec<u8>>) -> String {
    let mut boxes: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

    for seq in input.into_iter() {
        if seq.last().unwrap() == &b'-' {
            let label = &seq[0..seq.len() - 1];
            let box_num = hash(label);

            if let Some(v) = boxes.remove(&box_num) {
                let label_str = String::from_utf8(label.to_vec()).unwrap();

                boxes.insert(
                    box_num.clone(),
                    v.into_iter().filter(|(k, _)| k != &label_str).collect(),
                );
            }
        } else {
            let st = String::from_utf8(seq).unwrap();
            let (label, value) = st.split_once("=").unwrap();
            let box_num = hash(label.as_bytes());
            let value: usize = value.parse().unwrap();

            let mut v = if let Some(v) = boxes.remove(&box_num) {
                v
            } else {
                vec![]
            };

            let mut contained = false;
            v = v
                .into_iter()
                .map(|(k, v)| {
                    if k == label {
                        contained = true;
                        (k, value)
                    } else {
                        (k, v)
                    }
                })
                .collect();

            if !contained {
                v.push((label.to_string(), value));
            }

            boxes.insert(box_num, v);
        }
    }

    let mut sum = 0;
    for (k, v) in boxes.into_iter() {
        for (slot, (_, val)) in v.into_iter().enumerate() {
            sum += (1 + k) * (slot + 1) * val;
        }
    }

    sum.to_string()
}

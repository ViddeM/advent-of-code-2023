use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    outputs: Vec<String>,
    module_type: ModuleType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleType {
    Brodcaster,
    Conjunction,
    FlipFlop,
}

pub fn parse<'a>(input: &str) -> HashMap<String, Module> {
    input
        .lines()
        .map(|l| {
            let (input, output) = l.split_once(" -> ").unwrap();
            let (module_type, name) = if input == "broadcaster" {
                (ModuleType::Brodcaster, input.to_string())
            } else if let Some(name) = input.strip_prefix("&") {
                (ModuleType::Conjunction, name.to_string())
            } else if let Some(name) = input.strip_prefix("%") {
                (ModuleType::FlipFlop, name.to_string())
            } else {
                panic!("Invalid input {input:?}");
            };

            let outputs = output.split(",").map(|s| s.trim().to_string()).collect();

            (
                name.clone(),
                Module {
                    name,
                    outputs,
                    module_type,
                },
            )
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
enum Pulse {
    High,
    Low,
}

fn find_conjunction_inputs(
    modules: &HashMap<String, Module>,
) -> HashMap<&str, HashMap<&str, Pulse>> {
    let mut map: HashMap<&str, HashMap<&str, Pulse>> = HashMap::new();

    for (m, cont) in modules.iter() {
        if cont.module_type == ModuleType::Conjunction {
            map.insert(m, HashMap::new());
        }
    }

    for (m, cont) in modules.iter() {
        for out in cont.outputs.iter() {
            if let Some(l) = map.get_mut(out.as_str()) {
                l.insert(m, Pulse::Low);
            }
        }
    }

    map
}

fn find_flip_flop_states(modules: &HashMap<String, Module>) -> HashMap<&str, bool> {
    let mut map: HashMap<&str, bool> = HashMap::new();

    for (m, cont) in modules.iter() {
        if cont.module_type == ModuleType::FlipFlop {
            map.insert(m, false);
        }
    }

    map
}

fn simulate(
    modules: &HashMap<String, Module>,
    conjunction_inputs: &mut HashMap<&str, HashMap<&str, Pulse>>,
    flip_flop_states: &mut HashMap<&str, bool>,
    iteration: usize,
    rx_inputs: &mut HashMap<&str, usize>,
) -> (usize, usize) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut stack = vec![("button", "broadcaster", Pulse::Low)];

    while stack.len() > 0 {
        let (from, to, pulse) = stack.remove(0);
        // println!("{from} -{pulse:?}-> {to}");
        match pulse {
            Pulse::High => high_pulses += 1,
            Pulse::Low => {
                low_pulses += 1;
                if rx_inputs.contains_key(to) && rx_inputs[to] == 0 {
                    *rx_inputs.get_mut(to).unwrap() = iteration;
                }
            }
        };

        let Some(cont) = modules.get(to) else {
            continue;
        };

        let out_pulse = match cont.module_type {
            ModuleType::Brodcaster => Some(pulse),
            ModuleType::Conjunction => {
                let inps = conjunction_inputs.get_mut(to).unwrap();
                *inps.get_mut(from).unwrap() = pulse;
                if inps.values().all(|p| p == &Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            ModuleType::FlipFlop => {
                if pulse == Pulse::Low {
                    let state = flip_flop_states[to];
                    *flip_flop_states.get_mut(to).unwrap() = !state;

                    Some(if state { Pulse::Low } else { Pulse::High })
                } else {
                    None
                }
            }
        };

        if let Some(p) = out_pulse {
            for out in cont.outputs.iter() {
                stack.push((to, out, p.clone()));
            }
        }
    }

    (low_pulses, high_pulses)
}

pub fn solve_part_one<'a>(input: HashMap<String, Module>) -> String {
    let mut conjunction_inputs = find_conjunction_inputs(&input);
    let mut flip_flop_state = find_flip_flop_states(&input);
    let (mut lows, mut highs) = (0, 0);

    let mut ignored = HashMap::new();

    for i in 1..=1000 {
        let (ls, hs) = simulate(
            &input,
            &mut conjunction_inputs,
            &mut flip_flop_state,
            i,
            &mut ignored,
        );
        lows += ls;
        highs += hs;
    }

    // println!("LOWS {lows} HIGHS {highs}");
    (lows * highs).to_string()
}

fn get_rx_inputs(modules: &HashMap<String, Module>) -> HashMap<&str, usize> {
    let direct_input = modules
        .values()
        .filter(|v| v.outputs.contains(&String::from("rx")))
        .map(|v| v.name.as_str())
        .collect::<Vec<&str>>();

    if direct_input.len() != 1 {
        panic!(
            "Solution doesn't work for input, got {} direct inputs, expected 1",
            direct_input.len()
        );
    }

    let direct_input = direct_input[0];
    modules
        .values()
        .filter(|v| v.outputs.contains(&direct_input.to_string()))
        .map(|v| (v.name.as_str(), 0))
        .collect()
}

pub fn solve_part_two<'a>(input: HashMap<String, Module>) -> String {
    let mut conjunction_inputs = find_conjunction_inputs(&input);
    let mut flip_flop_state = find_flip_flop_states(&input);

    let mut rx_inputs = get_rx_inputs(&input);

    for i in 1..=usize::MAX {
        simulate(
            &input,
            &mut conjunction_inputs,
            &mut flip_flop_state,
            i,
            &mut rx_inputs,
        );

        if rx_inputs.values().all(|n| n > &0) {
            break;
        }
    }

    let ans = rx_inputs.values().fold(1, |acc, v| acc * v);
    ans.to_string()
}

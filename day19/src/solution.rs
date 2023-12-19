use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Workshop {
    workflows: HashMap<String, Workflow>,
    ratings: Vec<Part>,
}

#[derive(Debug, Clone)]
pub struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
pub struct Workflow {
    name: String,
    steps: Vec<(Cond, Action)>,
    default: Action,
}

#[derive(Debug, Clone)]
pub enum Action {
    Reject,
    Accept,
    SendTo(String),
}

impl Action {
    fn parse(input: &str) -> Self {
        match input {
            "A" => Self::Accept,
            "R" => Self::Reject,
            v => Self::SendTo(v.to_string()),
        }
    }

    fn to_workflow_name(&self) -> &str {
        match self {
            Action::Reject => "R",
            Action::Accept => "A",
            Action::SendTo(to) => to,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cond {
    reg: String,
    val: usize,
    is_greater_than: bool, // If true is greater than, otherwise is less than.
}

pub fn parse<'a>(input: &str) -> Workshop {
    let (workflows, ratings) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|workflow| {
            let (name, rest) = workflow.split_once("{").unwrap();

            let rest = rest.strip_suffix("}").unwrap();

            let mut steps = rest
                .split(",")
                .map(|a| a.to_string())
                .collect::<Vec<String>>();

            let default = steps.pop().unwrap();
            let default = Action::parse(&default);

            let steps = steps
                .into_iter()
                .map(|s| {
                    let (reg, rest, is_greater_than) = if let Some((reg, cond)) = s.split_once("<")
                    {
                        (reg, cond, false)
                    } else if let Some((reg, cond)) = s.split_once(">") {
                        (reg, cond, true)
                    } else {
                        panic!("Invalid step {s}");
                    };

                    let (num, action) = rest.split_once(":").unwrap();

                    let val = num.parse::<usize>().unwrap();

                    let action = Action::parse(action);

                    (
                        Cond {
                            reg: reg.to_string(),
                            val,
                            is_greater_than,
                        },
                        action,
                    )
                })
                .collect();

            Workflow {
                name: name.to_string(),
                steps,
                default,
            }
        })
        .map(|w| (w.name.clone(), w))
        .collect();

    let ratings = ratings
        .lines()
        .map(|rating| {
            let rating = rating.strip_prefix("{").unwrap().strip_suffix("}").unwrap();

            let rating = rating
                .split(",")
                .map(|r| {
                    let (reg, num) = r.split_once("=").unwrap();

                    (reg.to_string(), num.parse::<usize>().unwrap())
                })
                .collect::<HashMap<String, usize>>();

            Part {
                x: rating["x"],
                m: rating["m"],
                a: rating["a"],
                s: rating["s"],
            }
        })
        .collect();

    Workshop { workflows, ratings }
}

pub fn solve_part_one<'a>(input: Workshop) -> String {
    let mut sum = 0;

    'ratings: for rating in input.ratings.iter() {
        let mut curr_reg_name = "in";
        'reg_loop: loop {
            let curr_reg = input.workflows.get(curr_reg_name).unwrap();
            for (cond, action) in curr_reg.steps.iter() {
                let compare_val = match cond.reg.as_str() {
                    "x" => rating.x,
                    "m" => rating.m,
                    "a" => rating.a,
                    "s" => rating.s,
                    _ => panic!("Invalid reg {cond:?}"),
                };

                let accepted = if cond.is_greater_than {
                    compare_val > cond.val
                } else {
                    compare_val < cond.val
                };

                if accepted {
                    match action {
                        Action::Reject => continue 'ratings, // The part is rejected
                        Action::Accept => {
                            sum += rating.value();
                            continue 'ratings;
                        }
                        Action::SendTo(reg) => {
                            curr_reg_name = reg.as_str();
                            continue 'reg_loop;
                        }
                    }
                }
            }

            match &curr_reg.default {
                Action::Reject => continue 'ratings, // The part is rejected
                Action::Accept => {
                    sum += rating.value();
                    continue 'ratings;
                }
                Action::SendTo(reg) => {
                    curr_reg_name = reg.as_str();
                    continue 'reg_loop;
                }
            }
        }
    }

    sum.to_string()
}

#[derive(Debug, Clone)]
struct RegRanges {
    min_x: usize,
    max_x: usize,
    min_m: usize,
    max_m: usize,
    min_a: usize,
    max_a: usize,
    min_s: usize,
    max_s: usize,
}

impl RegRanges {
    fn is_valid(&self) -> bool {
        self.min_x <= self.max_x
            && self.min_m <= self.max_m
            && self.min_a <= self.max_a
            && self.min_s <= self.max_s
    }

    fn to_val(&self) -> usize {
        (self.max_x - self.min_x + 1)
            * (self.max_m - self.min_m + 1)
            * (self.max_a - self.min_a + 1)
            * (self.max_s - self.min_s + 1)
    }

    fn initial() -> Self {
        Self {
            min_x: 1,
            max_x: 4000,
            min_m: 1,
            max_m: 4000,
            min_a: 1,
            max_a: 4000,
            min_s: 1,
            max_s: 4000,
        }
    }
}

pub fn solve_part_two<'a>(input: Workshop) -> String {
    let mut states: Vec<(RegRanges, &str, usize)> = vec![(RegRanges::initial(), "in", 0)];

    let mut accepted_ranges: Vec<RegRanges> = vec![];

    while let Some((regs, workflow_name, step_index)) = states.pop() {
        if workflow_name == "A" {
            accepted_ranges.push(regs);
            continue;
        } else if workflow_name == "R" {
            continue;
        }

        if !regs.is_valid() {
            continue;
        }

        let workflow = input.workflows.get(workflow_name).unwrap();

        // Handle default
        if step_index >= workflow.steps.len() {
            match &workflow.default {
                Action::Reject => continue,
                Action::Accept => {
                    accepted_ranges.push(regs);
                    continue;
                }
                Action::SendTo(v) => {
                    states.push((regs, v.as_str(), 0));
                    continue;
                }
            }
        }

        let (cond, action) = &workflow.steps[step_index];
        if cond.is_greater_than {
            match cond.reg.as_str() {
                "x" => {
                    let mut lower_range = regs.clone();
                    lower_range.min_x = cond.val + 1;
                    states.push((lower_range, action.to_workflow_name(), 0));

                    let mut higher_range = regs.clone();
                    higher_range.max_x = cond.val;
                    states.push((higher_range, workflow_name, step_index + 1));
                }
                "m" => {
                    let mut lower_range = regs.clone();
                    lower_range.min_m = cond.val + 1;
                    states.push((lower_range, action.to_workflow_name(), 0));

                    let mut higher_range = regs.clone();
                    higher_range.max_m = cond.val;
                    states.push((higher_range, workflow_name, step_index + 1));
                }
                "a" => {
                    let mut lower_range = regs.clone();
                    lower_range.min_a = cond.val + 1;
                    states.push((lower_range, action.to_workflow_name(), 0));

                    let mut higher_range = regs.clone();
                    higher_range.max_a = cond.val;
                    states.push((higher_range, workflow_name, step_index + 1));
                }
                "s" => {
                    let mut lower_range = regs.clone();
                    lower_range.min_s = cond.val + 1;
                    states.push((lower_range, action.to_workflow_name(), 0));

                    let mut higher_range = regs.clone();
                    higher_range.max_s = cond.val;
                    states.push((higher_range, workflow_name, step_index + 1));
                }
                _ => panic!("Invalid cond {cond:?}"),
            }
        } else {
            match cond.reg.as_str() {
                "x" => {
                    let mut lower_range = regs.clone();
                    lower_range.max_x = cond.val - 1;
                    states.push((lower_range, action.to_workflow_name(), 0));

                    let mut higher_range = regs.clone();
                    higher_range.min_x = cond.val;
                    states.push((higher_range, workflow_name, step_index + 1));
                }
                "m" => {
                    let mut lower_range = regs.clone();
                    lower_range.max_m = cond.val - 1;
                    states.push((lower_range, action.to_workflow_name(), 0));

                    let mut higher_range = regs.clone();
                    higher_range.min_m = cond.val;
                    states.push((higher_range, workflow_name, step_index + 1));
                }
                "a" => {
                    let mut lower_range = regs.clone();
                    lower_range.max_a = cond.val - 1;
                    states.push((lower_range, action.to_workflow_name(), 0));

                    let mut higher_range = regs.clone();
                    higher_range.min_a = cond.val;
                    states.push((higher_range, workflow_name, step_index + 1));
                }
                "s" => {
                    let mut lower_range = regs.clone();
                    lower_range.max_s = cond.val - 1;
                    states.push((lower_range, action.to_workflow_name(), 0));

                    let mut higher_range = regs.clone();
                    higher_range.min_s = cond.val;
                    states.push((higher_range, workflow_name, step_index + 1));
                }
                _ => panic!("Invalid cond {cond:?}"),
            }
        }
    }

    accepted_ranges
        .iter()
        .map(|range| range.to_val())
        .sum::<usize>()
        .to_string()
}

use crate::utils::read_lines;
use std::collections::HashMap;
use std::str::FromStr;

pub fn day() {
    dayp1();
    dayp2();
}

fn dayp1() {
    let mut instructions = true;
    let mut workflows = HashMap::new();
    let mut total = 0;
    if let Ok(lines) = read_lines("data/day19.txt") {
        for line in lines {
            let Ok(ip) = line else { todo!() };
            if ip == "" {
                instructions = false;
            } else if instructions {
                let Some((name, flow1)) = ip.split_once('{') else {
                    todo!()
                };
                let flow: Vec<Instruction> = flow1
                    .trim_end_matches('}')
                    .split(',')
                    .map(|s| {
                        if s == "A" {
                            return Instruction::Approve;
                        }
                        if s == "R" {
                            return Instruction::Reject;
                        }
                        if let Some((comp, lbl)) = s.split_once(':') {
                            if let Some((t, amt)) = comp.split_once('<') {
                                return Instruction::Less(
                                    parse(t),
                                    amt.parse().unwrap(),
                                    lbl.to_string(),
                                );
                            }
                            if let Some((t, amt)) = comp.split_once('>') {
                                return Instruction::Greater(
                                    parse(t),
                                    amt.parse().unwrap(),
                                    lbl.to_string(),
                                );
                            }
                        }
                        return Instruction::GoTo(s.to_string());
                    })
                    .collect();
                workflows.insert(name.to_owned(), flow);
            } else {
                let part = ip.parse().unwrap();
                if follow(&part, &workflows, "in") {
                    total += part.x + part.m + part.a + part.s;
                }
            }
        }
    }
    println!("day 19 part 1: {}", total);
}

fn dayp2() {
    let mut instructions = true;
    let mut workflows = HashMap::new();
    let mut total = 0;
    if let Ok(lines) = read_lines("data/day19.txt") {
        for line in lines {
            let Ok(ip) = line else { todo!() };
            if ip == "" {
                instructions = false;
            } else if instructions {
                let Some((name, flow1)) = ip.split_once('{') else {
                    todo!()
                };
                let flow: Vec<Instruction> = flow1
                    .trim_end_matches('}')
                    .split(',')
                    .map(|s| {
                        if s == "A" {
                            return Instruction::Approve;
                        }
                        if s == "R" {
                            return Instruction::Reject;
                        }
                        if let Some((comp, lbl)) = s.split_once(':') {
                            if let Some((t, amt)) = comp.split_once('<') {
                                return Instruction::Less(
                                    parse(t),
                                    amt.parse().unwrap(),
                                    lbl.to_string(),
                                );
                            }
                            if let Some((t, amt)) = comp.split_once('>') {
                                return Instruction::Greater(
                                    parse(t),
                                    amt.parse().unwrap(),
                                    lbl.to_string(),
                                );
                            }
                        }
                        return Instruction::GoTo(s.to_string());
                    })
                    .collect();
                workflows.insert(name.to_owned(), flow);
            }
        }
    }
    println!(
        "day 19 part 2: {}",
        follow_ranges(
            &mut RangePart {
                x: (1, 4000),
                m: (1, 4000),
                a: (1, 4000),
                s: (1, 4000)
            },
            &workflows,
            "in"
        )
    );
}

fn follow_ranges(
    range: &mut RangePart,
    workflows: &HashMap<String, Vec<Instruction>>,
    label: &str,
) -> usize {
    if label == "A" {
        return size(range);
    }
    if label == "R" {
        return 0;
    }
    let Some(workflow) = workflows.get(label) else {
        todo!()
    };
    let mut total = 0;
    for inst in workflow {
        match inst {
            Instruction::GoTo(s) => total += follow_ranges(range, workflows, s),
            Instruction::Approve => total += size(range),
            Instruction::Reject => total += 0,
            Instruction::Greater(v, re, go) => {
                let (irl, irh) = item_range(range, v);
                if irl > *re {
                    total += follow_ranges(range, workflows, go);
                } else if irh < *re {
                    total += 0;
                } else {
                    let mut np = range.clone();
                    let opv = cp(range, v);
                    *opv = (irl, *re);
                    let npv = cp(&mut np, v);
                    *npv = (*re+1, irh);
                    total += follow_ranges(&mut np, workflows, go);
                }
            }
            Instruction::Less(v, re, go) => {
                let (irl, irh) = item_range(range, v);
                if irh < *re {
                    total += follow_ranges(range, workflows, go);
                } else if irl > *re {
                    total += 0;
                } else {
                    let mut np = range.clone();
                    let opv = cp(range, v);
                    *opv = (*re, irh);
                    let npv = cp(&mut np, v);
                    *npv = (irl, *re-1);
                    total += follow_ranges(&mut np, workflows, go);
                }
            }
        }
    }

    return total;
}

fn cp<'a>(range: &'a mut RangePart, item: &Item) -> &'a mut (usize, usize) {
    match item {
        Item::ExtremelyCool => &mut range.x,
        Item::Musical => &mut range.m,
        Item::Aerodynamic => &mut range.a,
        Item::Shiny => &mut range.s,
    }
}

fn size(
    RangePart {
        x: (x1, x2),
        m: (m1, m2),
        a: (a1, a2),
        s: (s1, s2),
    }: &RangePart,
) -> usize {
    return (x2 - x1 + 1) * (m2 - m1 + 1) * (a2 - a1 + 1) * (s2 - s1 + 1);
}

fn follow(p: &Part, instructions: &HashMap<String, Vec<Instruction>>, label: &str) -> bool {
    let Some(workflow) = instructions.get(label) else {
        todo!()
    };
    for inst in workflow {
        match inst {
            Instruction::GoTo(s) => return follow(p, instructions, s),
            Instruction::Approve => return true,
            Instruction::Reject => return false,
            Instruction::Greater(v, re, go) => {
                if item(p, v) > *re {
                    if go == "A" {
                        return true;
                    }
                    if go == "R" {
                        return false;
                    }
                    return follow(p, instructions, go);
                }
            }
            Instruction::Less(v, re, go) => {
                if item(p, v) < *re {
                    if go == "A" {
                        return true;
                    }
                    if go == "R" {
                        return false;
                    }
                    return follow(p, instructions, go);
                }
            }
        }
    }
    todo!();
}

fn item_range(RangePart { x, m, a, s }: &RangePart, item: &Item) -> (usize, usize) {
    match item {
        Item::ExtremelyCool => return *x,
        Item::Musical => return *m,
        Item::Aerodynamic => return *a,
        Item::Shiny => return *s,
    };
}

fn item(Part { x, m, a, s }: &Part, item: &Item) -> usize {
    match item {
        Item::ExtremelyCool => return *x,
        Item::Musical => return *m,
        Item::Aerodynamic => return *a,
        Item::Shiny => return *s,
    };
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Clone, Copy)]
struct RangePart {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

#[derive(Debug)]
enum Instruction {
    GoTo(String),
    Less(Item, usize, String),
    Greater(Item, usize, String),
    Approve,
    Reject,
}

#[derive(Debug)]
enum Item {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

fn parse(s: &str) -> Item {
    return match s {
        "x" => Item::ExtremelyCool,
        "m" => Item::Musical,
        "a" => Item::Aerodynamic,
        "s" => Item::Shiny,
        _ => todo!(),
    };
}

impl FromStr for Part {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for p in s
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
        {
            let Some((n, amt)) = p.split_once('=') else {
                todo!()
            };
            match n {
                "x" => part.x = amt.parse().unwrap(),
                "m" => part.m = amt.parse().unwrap(),
                "a" => part.a = amt.parse().unwrap(),
                "s" => part.s = amt.parse().unwrap(),
                _ => return Err("part was not xmas".to_string()),
            }
        }
        return Ok(part);
    }
}

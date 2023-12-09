use std::{collections::HashMap, i128, str::FromStr};

use crate::utils::read_lines;

pub fn day() {
    dayp1();
    dayp2();
}

fn dayp1() {
    if let Ok(mut lines) = read_lines("data/day8.txt") {
        if let Some(Ok(choice_line)) = lines.next() {
            let mut nodes = HashMap::new();
            let choices: Vec<Choice> = choice_line
                .chars()
                .map(|char| char.to_string().parse().unwrap())
                .collect();
            lines.next();
            for line in lines {
                if let Ok(ip) = line {
                    if let Some((start, tuple)) = ip.split_once(" = (") {
                        if let Some((fst, snd)) =
                            tuple.strip_suffix(")").unwrap_or(tuple).split_once(", ")
                        {
                            nodes.insert(
                                String::from(start),
                                (String::from(fst), String::from(snd)),
                            );
                        }
                    }
                }
            }
            let path = choices
                .iter()
                .cycle()
                .try_fold(("AAA", 0), |(acc, cnt), dir| {
                    if acc == "ZZZ" {
                        return Err(cnt);
                    }
                    let (left_path, right_path) = &nodes[acc];
                    match dir {
                        Choice::Left => return Ok((left_path.as_str(), cnt + 1)),
                        Choice::Right => return Ok((right_path.as_str(), cnt + 1)),
                    }
                });
            println!("day 8 part 1: {}", path.unwrap_err());
        }
    }
}

fn dayp2() {
    if let Ok(mut lines) = read_lines("data/day8.txt") {
        if let Some(Ok(choice_line)) = lines.next() {
            let mut nodes = HashMap::new();
            let mut current_nodes: Vec<String> = Vec::new();
            let choices: Vec<Choice> = choice_line
                .chars()
                .map(|char| char.to_string().parse().unwrap())
                .collect();
            lines.next();
            for line in lines {
                if let Ok(ip) = line {
                    if let Some((start, tuple)) = ip.split_once(" = (") {
                        if let Some((fst, snd)) =
                            tuple.strip_suffix(")").unwrap_or(tuple).split_once(", ")
                        {
                            nodes.insert(
                                String::from(start),
                                (String::from(fst), String::from(snd)),
                            );
                        }
                        if start.ends_with("A") {
                            current_nodes.push(String::from(start));
                        }
                    }
                }
            }
            let mut total = 1;
            for start in current_nodes {
                let path: Vec<(String, i128)> = choices
                    .iter()
                    .cycle()
                    .scan((start, 0), |(acc, cnt), dir| {
                        *cnt += 1;
                        if acc.ends_with('Z') {
                            return None;
                        }
                        let (left_path, right_path) = &nodes[acc];
                        match dir {
                            Choice::Left => {
                                *acc = left_path.clone();
                                return Some((left_path.to_string(), *cnt));
                            }
                            Choice::Right => {
                                *acc = right_path.clone();
                                return Some((right_path.to_string(), *cnt));
                            }
                        }
                    })
                    .collect();
                total = lcm(
                    path.last().unwrap().1, //whyyyy
                    total,
                );
            }
            println!("day 8 part 2: {}", total);
        }
    }
}

#[derive(Debug)]
enum Choice {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct ChoiceErr;

impl FromStr for Choice {
    type Err = ChoiceErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Choice::Right),
            "L" => Ok(Choice::Left),
            _ => {
                println!("err with {}", s);
                Err(ChoiceErr)
            }
        }
    }
}
fn lcm(first: i128, second: i128) -> i128 {
    first * second / gcd(first, second)
}

fn gcd(first: i128, second: i128) -> i128 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

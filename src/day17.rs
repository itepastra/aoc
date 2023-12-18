use crate::utils::read_lines;
use priority_queue::DoublePriorityQueue;
use std::collections::HashMap;

pub fn day() {
    dayp1();
    dayp2();
}

fn dayp1() {
    let max = 3;
    let mut nodes = HashMap::new();
    let mut unvisited = DoublePriorityQueue::new();
    let mut goal = (0, 0);
    if let Ok(lines) = read_lines("data/day17.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(ip) = line {
                ip.chars().enumerate().for_each(|(x, c)| {
                    let n = char_to_num(c);
                    for i in 1..max + 1 {
                        for d in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                            nodes.insert((x, y, d, i), (n, usize::MAX));
                        }
                    }
                });
                goal.0 = ip.len() - 1;
            }
            goal.1 = y
        }
    }
    let start = (0, 0, Dir::Right, 0);
    nodes.insert(start, (0, 0));
    unvisited.push(start, 0);

    while unvisited.len() > 0 {
        let (curr, ct) = unvisited.pop_min().unwrap();
        if curr.0 == goal.0 && curr.1 == goal.1 {
            println!("day 17 part 1: {}", ct);
            break;
        }
        unvisited.remove(&curr);
        for neigh in get_neighbours_p1(curr).iter() {
            match nodes.get_mut(neigh) {
                Some((a, b)) => {
                    if *a + ct < *b {
                        *b = *a + ct;
                        unvisited.push_decrease(*neigh, *a + ct);
                    }
                }
                _ => {}
            }
        }
    }
}

fn dayp2() {
    let max = 10;
    let mut nodes = HashMap::new();
    let mut unvisited = DoublePriorityQueue::new();
    let mut goal = (0, 0);
    if let Ok(lines) = read_lines("data/day17.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(ip) = line {
                ip.chars().enumerate().for_each(|(x, c)| {
                    let n = char_to_num(c);
                    for i in 1..max + 1 {
                        for d in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                            nodes.insert((x, y, d, i), (n, usize::MAX));
                            unvisited.push((x, y, d, i), usize::MAX);
                        }
                    }
                });
                goal.0 = ip.len() - 1;
            }
            goal.1 = y
        }
    }
    let start = (0, 0, Dir::Right, 0);
    nodes.insert(start, (0, 0));
    let start2 = (0, 0, Dir::Down, 0); // we can't go down the first column if we dont start going
                                       // down
    nodes.insert(start2, (0, 0));
    unvisited.push(start, 0);

    while unvisited.len() > 0 {
        let (curr, ct) = unvisited.pop_min().unwrap();
        if curr.0 == goal.0 && curr.1 == goal.1 && curr.3 >= 4 {
            println!("day 17 part 2: {}", ct);
            break;
        }
        unvisited.remove(&curr);
        for neigh in get_neighbours_p2(curr).iter() {
            match nodes.get_mut(neigh) {
                Some((a, b)) => {
                    if *a + ct < *b {
                        *b = *a + ct;
                        unvisited.push_decrease(*neigh, *b);
                    }
                }
                _ => {}
            }
        }
    }
}

fn get_neighbours_p2((x, y, dir, l): Node) -> Vec<Node> {
    let min = 4;
    let mut v = Vec::new();
    match dir {
        Dir::Up => {
            if y > 0 {
                v.push((x, y - 1, Dir::Up, l + 1));
            }
            if l >= min {
                if x > 0 {
                    v.push((x - 1, y, Dir::Left, 1));
                }
                v.push((x + 1, y, Dir::Right, 1));
            }
        }
        Dir::Down => {
            v.push((x, y + 1, Dir::Down, l + 1));
            if l >= min {
                v.push((x + 1, y, Dir::Right, 1));
                if x > 0 {
                    v.push((x - 1, y, Dir::Left, 1));
                }
            }
        }
        Dir::Left => {
            if x > 0 {
                v.push((x - 1, y, Dir::Left, l + 1));
            }
            if l >= min {
                v.push((x, y + 1, Dir::Down, 1));
                if y > 0 {
                    v.push((x, y - 1, Dir::Up, 1));
                }
            }
        }
        Dir::Right => {
            v.push((x + 1, y, Dir::Right, l + 1));
            if l >= min {
                v.push((x, y + 1, Dir::Down, 1));
                if y > 0 {
                    v.push((x, y - 1, Dir::Up, 1));
                }
            }
        }
    };
    return v;
}

fn get_neighbours_p1((x, y, dir, l): Node) -> Vec<Node> {
    let mut v = Vec::new();
    match dir {
        Dir::Up => {
            if y > 0 {
                v.push((x, y - 1, Dir::Up, l + 1));
            }
            if x > 0 {
                v.push((x - 1, y, Dir::Left, 1));
            }
            v.push((x + 1, y, Dir::Right, 1));
        }
        Dir::Down => {
            v.push((x, y + 1, Dir::Down, l + 1));
            v.push((x + 1, y, Dir::Right, 1));
            if x > 0 {
                v.push((x - 1, y, Dir::Left, 1));
            }
        }
        Dir::Left => {
            if x > 0 {
                v.push((x - 1, y, Dir::Left, l + 1));
            }
            v.push((x, y + 1, Dir::Down, 1));
            if y > 0 {
                v.push((x, y - 1, Dir::Up, 1));
            }
        }
        Dir::Right => {
            v.push((x + 1, y, Dir::Right, l + 1));
            v.push((x, y + 1, Dir::Down, 1));
            if y > 0 {
                v.push((x, y - 1, Dir::Up, 1));
            }
        }
    };
    return v;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

type Node = (usize, usize, Dir, usize);

fn char_to_num(c: char) -> usize {
    return match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => usize::MAX,
    };
}

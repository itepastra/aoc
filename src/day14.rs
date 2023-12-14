use crate::utils::read_lines;
use std::collections::HashMap;

pub fn day() {
    dayp1();
    dayp2();
}

fn dayp1() {
    let mut rocks: Vec<Vec<Rock>> = Vec::new();
    if let Ok(lines) = read_lines("data/day14.txt") {
        for (j, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let line_rocks: Vec<_> = ip.chars().map(|c| Rock::try_from(c).unwrap()).collect();
                rocks.push(line_rocks.clone());
                let _y: Vec<_> = line_rocks
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| **r == Rock::Rolling)
                    .map(|(i, _)| roll_rock(&mut rocks, (i, j)))
                    .collect();
            }
        }
    }
    println!("day 14 part 1: {}", score(&rocks));
}

fn dayp2() {
    let mut cache = HashMap::new();

    let mut rocks: Vec<Vec<Rock>> = Vec::new();
    if let Ok(lines) = read_lines("data/day14.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let line_rocks: Vec<_> = ip.chars().map(|c| Rock::try_from(c).unwrap()).collect();
                rocks.push(line_rocks.clone());
            }
        }
    }

    let amt = 1000000000;
    let mut n = 0;
    let mut cycle_length;
    while n < amt {
        let RockLoop {
            rocks: nrocks,
            in_loop: in_cycle,
        } = cycle_rocks(&mut rocks, n, &mut cache);
        if let Some(cyc) = in_cycle {
            cycle_length = n - cyc;
            n += ((amt - n)/cycle_length) * cycle_length + 1;
            cache = HashMap::new();
        } else { 
            n += 1; 
        }
        rocks = nrocks;
    }
    println!("day 14 part 2: {}", score(&rocks));
}

fn score(rocks: &Vec<Vec<Rock>>) -> usize {
    let mut total = 0;
    for (i, row) in rocks.iter().enumerate() {
        let y = rocks.len() - i;
        total += row
            .iter()
            .map(|r| match r {
                Rock::Rolling => y,
                _ => 0,
            })
            .sum::<usize>();
    }
    return total;
}

fn pprint(rocks: &Vec<Vec<Rock>>) {
    for row in rocks {
        println!(
            "{}",
            row.iter()
                .map(|r| match r {
                    Rock::Rolling => 'O',
                    Rock::Static => '#',
                    Rock::Empty => '.',
                })
                .collect::<String>()
        );
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Rock {
    Rolling,
    Static,
    Empty,
}

struct RockLoop {
    rocks: Vec<Vec<Rock>>,
    in_loop: Option<usize>,
}

impl TryFrom<char> for Rock {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Rock::Static),
            'O' => Ok(Rock::Rolling),
            '.' => Ok(Rock::Empty),
            _ => Err("char did not exist"),
        }
    }
}

fn roll_rock(rocks: &mut Vec<Vec<Rock>>, (x, y): (usize, usize)) {
    if rocks[y][x] != Rock::Rolling {
        return;
    }
    let mut cy = y;
    while cy > 0 && rocks[cy - 1][x] == Rock::Empty {
        (rocks[cy - 1][x], rocks[cy][x]) = (rocks[cy][x], rocks[cy - 1][x]);
        cy -= 1
    }
}

fn roll_rocks_up(rocks: &mut Vec<Vec<Rock>>) {
    for x in 0..rocks[0].len() {
        for y in 0..rocks.len() {
            roll_rock(rocks, (x, y));
        }
    }
}

fn rotate(rocks: &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
    let mut result = vec![vec![Rock::Static; rocks.len()]; rocks[0].len()];
    for x in 0..rocks[0].len() {
        for y in 0..rocks.len() {
            result[x][rocks[0].len() - 1 - y] = rocks[y][x];
        }
    }

    return result;
}

fn cycle_rocks(
    rocks: &Vec<Vec<Rock>>,
    num: usize,
    cache: &mut HashMap<Vec<Vec<Rock>>, (Vec<Vec<Rock>>, usize)>,
) -> RockLoop {
    match cache.get(rocks) {
        Some((new_rocks, n)) => {
            return RockLoop {
                rocks: new_rocks.to_vec(),
                in_loop: Some(*n),
            };
        }
        None => {}
    };

    let mut new_rocks = rocks.clone();
    roll_rocks_up(&mut new_rocks);
    new_rocks = rotate(&new_rocks);
    roll_rocks_up(&mut new_rocks);
    new_rocks = rotate(&new_rocks);
    roll_rocks_up(&mut new_rocks);
    new_rocks = rotate(&new_rocks);
    roll_rocks_up(&mut new_rocks);
    new_rocks = rotate(&new_rocks);

    cache.insert(rocks.to_vec(), (new_rocks.clone(), num));

    return RockLoop {
        rocks: new_rocks,
        in_loop: None,
    };
}

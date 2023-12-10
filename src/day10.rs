use crate::utils::read_lines;
use std::str::FromStr;

pub fn day() {
    dayp1();
}

fn dayp1() {
    let mut grid = Vec::new();
    let mut start = (0, 0);
    if let Ok(lines) = read_lines("data/day10.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let v: Vec<Pipe> = ip
                    .chars()
                    .map(|c| c.to_string().parse().unwrap_or(Pipe::Ground))
                    .collect();
                if let Some(spos) = v.iter().position(|p| *p == Pipe::Start) {
                    start = (spos as i32, i as i32);
                }
                grid.push(v);
            }
        }
    }
    let mut lp = Vec::new();
    let mut found = false;
    let mut start_dir: (i32, i32) = (0, -1);
    let mut prev: (i32, i32) = start;
    while start_dir.0 + prev.0 < 0
        || start_dir.1 + prev.1 < 0
        || start_dir.0 + prev.0 >= grid[0].len() as i32
        || start_dir.1 + prev.1 >= grid[0].len() as i32
    {
        start_dir = (-start_dir.1, start_dir.0);
    }
    let mut current = (prev.0 + start_dir.0, prev.1 + start_dir.1);
    lp.push(current);
    while !found {
        if let Ok(next) = follow_pipe(&grid, current, prev) {
            lp.push(next);
            if next == current {
                println!("day 10 part 1: {}", lp.len() / 2);
                found = true;
            }
            prev = current;
            current = next;
        } else {
            start_dir = (-start_dir.1, start_dir.0);
            prev = start;
            current = (prev.0 + start_dir.0, prev.1 + start_dir.1);
            lp = Vec::new();
            lp.push(current);
        }
    }
    replace_s(&mut grid, &lp);
    let mut iomap: Vec<Vec<LoopOption>> = Vec::new();
    for (lidx, line) in grid.iter().enumerate().map(|(a, b)| (a as i32, b)) {
        let in_points: Vec<LoopOption> = line
            .iter()
            .enumerate()
            .map(|(a, _)| (a as i32, lidx))
            .rev()
            .scan(LoopOption::OUT, |acc, cur| {
                *acc = check_if_in_loop(cur, *acc, &lp, &grid);
                return Some(*acc);
            })
            .collect();
        iomap.push(in_points.into_iter().rev().collect());
    }
    let mut total_in = 0;
    for line in iomap {
        let ppline: String = line
            .iter()
            .map(|c| match c {
                LoopOption::IN => {
                    total_in += 1;
                    'I'
                }
                LoopOption::OUT => 'O',
                _ => ' ',
            })
            .collect();
        if false {
        println!("{}", ppline);
        }
    }
    println!("day 10 part 2: {}", total_in);
}

fn replace_s(grid: &mut Vec<Vec<Pipe>>, lop: &Vec<(i32, i32)>) {
    let first_non_start = lop[0];
    let len = lop.len();
    let last_non_start = lop[len - 3];
    let start = lop[len-1];
    let diff = (
        first_non_start.0 - last_non_start.0,
        first_non_start.1 - last_non_start.1,
    );
    let new_pipe = match diff {
        (1, -1) => Pipe::NorthWest,
        _ => todo!(),
    };
    grid[start.1 as usize][start.0 as usize] = new_pipe;
}

fn follow_pipe(
    grid: &Vec<Vec<Pipe>>,
    (cx, cy): (i32, i32),
    (px, py): (i32, i32),
) -> Result<(i32, i32), PipeError> {
    let cp = &grid[cy as usize][cx as usize];
    match cp {
        Pipe::Vertical if py > cy && px == cx => {
            return Ok((cx, cy - 1));
        }
        Pipe::Vertical if px == cx => {
            return Ok((cx, cy + 1));
        }
        Pipe::Horizontal if px > cx && py == cy => {
            return Ok((cx - 1, cy));
        }
        Pipe::Horizontal if py == cy => {
            return Ok((cx + 1, cy));
        }
        Pipe::NorthEast if py < cy && px == cx => {
            return Ok((cx + 1, cy));
        }
        Pipe::NorthEast if px > cx && py == cy => {
            return Ok((cx, cy - 1));
        }
        Pipe::NorthWest if py < cy && px == cx => {
            return Ok((cx - 1, cy)); // heading west
        }
        Pipe::NorthWest if px < cx && py == cy => {
            return Ok((cx, cy - 1)); // heading north
        }
        Pipe::SouthEast if py > cy && px == cx => {
            return Ok((cx + 1, cy));
        }
        Pipe::SouthEast if px > cx && py == cy => {
            return Ok((cx, cy + 1));
        }
        Pipe::SouthWest if py > cy && px == cx => {
            return Ok((cx - 1, cy));
        }
        Pipe::SouthWest if px < cx && py == cy => {
            return Ok((cx, cy + 1));
        }
        Pipe::Vertical
        | Pipe::Horizontal
        | Pipe::NorthEast
        | Pipe::NorthWest
        | Pipe::SouthEast
        | Pipe::SouthWest => {
            return Err(PipeError);
        }
        Pipe::Ground => return Err(PipeError),
        Pipe::Start => return Ok((cx, cy)),
    }
}

fn check_if_in_loop(
    point: (i32, i32),
    right_state: LoopOption,
    lop: &Vec<(i32, i32)>,
    grid: &Vec<Vec<Pipe>>,
) -> LoopOption {
    let (px, py) = point;
    let current_tile = &grid[py as usize][px as usize];
    match (current_tile, right_state, lop.iter().find(|p| **p == point)) {
        (Pipe::Vertical | Pipe::NorthEast | Pipe::NorthWest, rs, Some(_)) => return invert_on(rs),
        (_, rs, Some(_)) => return keep_on(rs),
        (_, rs, None) => return keep_off(rs),
    }
}

fn invert_on(loop_option: LoopOption) -> LoopOption {
    return match loop_option {
        LoopOption::IN | LoopOption::ONIN => LoopOption::ONOUT,
        LoopOption::OUT | LoopOption::ONOUT => LoopOption::ONIN,
    };
}

fn keep_on(loop_option: LoopOption) -> LoopOption {
    return match loop_option {
        LoopOption::IN | LoopOption::ONIN => LoopOption::ONIN,
        LoopOption::OUT | LoopOption::ONOUT => LoopOption::ONOUT,
    };
}
fn invert_off(loop_option: LoopOption) -> LoopOption {
    return match loop_option {
        LoopOption::IN | LoopOption::ONIN => LoopOption::OUT,
        LoopOption::OUT | LoopOption::ONOUT => LoopOption::IN,
    };
}

fn keep_off(loop_option: LoopOption) -> LoopOption {
    return match loop_option {
        LoopOption::IN | LoopOption::ONIN => LoopOption::IN,
        LoopOption::OUT | LoopOption::ONOUT => LoopOption::OUT,
    };
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum LoopOption {
    IN,
    OUT,
    ONOUT,
    ONIN,
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq)]
struct PipeError;

impl FromStr for Pipe {
    type Err = PipeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Pipe::Vertical),
            "-" => Ok(Pipe::Horizontal),
            "L" => Ok(Pipe::NorthEast),
            "J" => Ok(Pipe::NorthWest),
            "7" => Ok(Pipe::SouthWest),
            "F" => Ok(Pipe::SouthEast),
            "." => Ok(Pipe::Ground),
            "S" => Ok(Pipe::Start),
            _ => Err(PipeError),
        }
    }
}

use crate::utils::read_lines;
use std::collections::HashSet;
use std::convert::From;

pub fn day() {
    dayp1();
    dayp2();
}

fn dayp1() {
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    if let Ok(lines) = read_lines("data/day16.txt") {
        for line in lines {
            if let Ok(ip) = line {
                grid.push(ip.chars().map(|c| Tile::from(c)).collect());
            }
        }
    }
    let mut heads = vec![(0, 0, 1, 0)];
    let mut cache = HashSet::new();
    while heads.len() > 0 {
        step(&grid, &mut heads, &mut cache);
    }
    let fin: HashSet<_> = cache.iter().map(|(x, y, _, _)| (x, y)).collect();
    println!("day 16 part 1: {}", fin.len());
}

fn dayp2() {
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    if let Ok(lines) = read_lines("data/day16.txt") {
        for line in lines {
            if let Ok(ip) = line {
                grid.push(ip.chars().map(|c| Tile::from(c)).collect());
            }
        }
    }

    let max = starts(&grid)
        .iter()
        .map(|start| {
            let mut heads = vec![*start];
            let mut cache = HashSet::new();
            while heads.len() > 0 {
                step(&grid, &mut heads, &mut cache);
            }
            let fin: HashSet<_> = cache.iter().map(|(x, y, _, _)| (x, y)).collect();
            return fin.len();
        })
        .max().unwrap();
    println!("day 16 part 2: {}", max);
}

fn starts<T>(grid: &Vec<Vec<T>>) -> Vec<(usize, usize, i8, i8)> {
    let ymax = grid.len();
    let xmax = grid[0].len();
    let mut starts = Vec::new();
    for x in 0..xmax {
        starts.push((x, 0, 0, 1));
        starts.push((x, ymax - 1, 0, -1));
    }
    for y in 0..ymax {
        starts.push((0, y, 1, 0));
        starts.push((xmax - 1, y, -1, 0));
    }
    return starts;
}

fn step(
    grid: &Vec<Vec<Tile>>,
    heads: &mut Vec<(usize, usize, i8, i8)>,
    cache: &mut HashSet<(usize, usize, i8, i8)>,
) {
    let Some((x, y, dx, dy)) = heads.pop() else {
        todo!()
    };
    if y >= grid.len() || x >= grid[0].len() {
        return;
    }
    if cache.contains(&(x, y, dx, dy)) {
        return;
    }
    cache.insert((x, y, dx, dy));

    let tile = &grid[y][x];
    match tile {
        Tile::Empty => {
            if let Some(new) = add(x, y, dx, dy) {
                heads.push(new);
            }
        }
        Tile::SplitterVertical => {
            if dx == 0 {
                if let Some(new) = add(x, y, dx, dy) {
                    heads.push(new);
                }
            } else {
                if let Some(new) = add(x, y, 0, 1) {
                    heads.push(new);
                }
                if let Some(new) = add(x, y, 0, -1) {
                    heads.push(new);
                }
            }
        }
        Tile::SplitterHorizontal => {
            if dy == 0 {
                if let Some(new) = add(x, y, dx, dy) {
                    heads.push(new);
                }
            } else {
                if let Some(new) = add(x, y, 1, 0) {
                    heads.push(new);
                }
                if let Some(new) = add(x, y, -1, 0) {
                    heads.push(new);
                }
            }
        }
        Tile::MirrorRight => {
            if dy == 0 && dx > 0 {
                if let Some(new) = add(x, y, 0, -1) {
                    heads.push(new);
                }
            } else if dy == 0 && dx < 0 {
                if let Some(new) = add(x, y, 0, 1) {
                    heads.push(new);
                }
            } else if dx == 0 && dy > 0 {
                if let Some(new) = add(x, y, -1, 0) {
                    heads.push(new);
                }
            } else if dx == 0 && dy < 0 {
                if let Some(new) = add(x, y, 1, 0) {
                    heads.push(new);
                }
            }
        }
        Tile::MirrorLeft => {
            if dy == 0 && dx > 0 {
                if let Some(new) = add(x, y, 0, 1) {
                    heads.push(new);
                }
            } else if dy == 0 && dx < 0 {
                if let Some(new) = add(x, y, 0, -1) {
                    heads.push(new);
                }
            } else if dx == 0 && dy > 0 {
                if let Some(new) = add(x, y, 1, 0) {
                    heads.push(new);
                }
            } else if dx == 0 && dy < 0 {
                if let Some(new) = add(x, y, -1, 0) {
                    heads.push(new);
                }
            }
        }
    };
}

fn add(x: usize, y: usize, dx: i8, dy: i8) -> Option<(usize, usize, i8, i8)> {
    let nx;
    let ny;
    if dx >= 0 {
        nx = x + dx as usize;
    } else if x > 0 {
        nx = x - (-dx) as usize;
    } else {
        return None;
    };
    if dy >= 0 {
        ny = y + dy as usize;
    } else if y > 0 {
        ny = y - (-dy) as usize;
    } else {
        return None;
    }
    return Some((nx, ny, dx, dy));
}

#[derive(Debug)]
enum Tile {
    Empty,
    MirrorLeft,
    MirrorRight,
    SplitterVertical,
    SplitterHorizontal,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '|' => Tile::SplitterVertical,
            '-' => Tile::SplitterHorizontal,
            '/' => Tile::MirrorRight,
            '\\' => Tile::MirrorLeft,
            _ => Tile::Empty,
        }
    }
}

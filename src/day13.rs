use crate::utils::read_lines;
use std::str::FromStr;

pub fn day() {
    dayp1();
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Dot,
    Hash,
}

fn dayp1() {
    let mut grids: Vec<Grid> = Vec::new();
    if let Ok(lines) = read_lines("data/day13.txt") {
        let mut grd = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    grids.push(grd);
                    grd = Vec::new();
                } else {
                    grd.push(ip.chars().map(|c| c.to_string().parse().unwrap()).collect());
                }
            }
        }
    }
    let mut total = 0;
    let mut total2 = 0;
    for grid in grids {
        for offset in 0..grid[0].len() - 1 {
            let is_mirror_vert = check_if_mirror_vertical(&grid, offset);
            if is_mirror_vert {
                total += offset + 1;
            }
            if check_single_vertical(&grid, offset) {
                total2 += offset + 1;
            }
        }
        for offset in 0..grid.len() - 1 {
            let is_mirror_hori = check_if_mirror_horizontal(&grid, offset);
            if is_mirror_hori {
                total += (offset + 1) * 100;
            }
            if check_single_horizontal(&grid, offset) {
                total2 += (offset + 1) * 100;
            }
        }
    }
    println!("day 13 part 1: {}", total);
    println!("day 13 part 2: {}", total2);
}

fn count_diffs(a: &Vec<Tile>, b: &Vec<Tile>) -> usize
{
    return a
        .iter()
        .zip(b.iter())
        .fold(0, |acc, (i, j)| {
            acc + if i != j { 1 } else { 0 }});
}

fn check_single_horizontal(grid: &Grid, offset: usize) -> bool {
    let mut diffs = 0;
    for i in 0..offset + 1 {
        if diffs > 1 {
            return false;
        }
        if i + offset + 1 >= grid.len() {
            break;
        }
        diffs += count_diffs(&grid[offset + i + 1], &grid[offset - i]);
    }
    if diffs == 1 {
        return true;
    } else {
        return false;
    }
}

fn check_single_vertical(grid: &Grid, offset: usize,) -> bool {
    return if grid.iter().fold(0, |acc, line| {
        for i in 0..offset + 1 {
            if i + offset + 1 >= line.len() {
                break;
            }
            if line[offset + i + 1] != line[offset - i] {
                return acc + 1;
            }
        }
        return acc;
    }) == 1 {true} else {false};
}

fn check_if_mirror_horizontal(grid: &Grid, offset: usize) -> bool {
    for i in 0..offset + 1 {
        if i + offset + 1 >= grid.len() {
            break;
        }
        if grid[offset + i + 1] != grid[offset - i] {
            return false;
        }
    }
    return true;
}

fn check_if_mirror_vertical(grid: &Grid, offset: usize) -> bool {
    return grid.iter().all(|l| {
        for i in 0..offset + 1 {
            if i + offset + 1 >= l.len() {
                break;
            }
            if l[offset + i + 1] != l[offset - i] {
                return false;
            }
        }
        return true;
    });
}

#[derive(Debug, PartialEq, Eq)]
struct TileParseError;

impl FromStr for Tile {
    type Err = TileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "." => Ok(Tile::Dot),
            "#" => Ok(Tile::Hash),
            _ => Err(TileParseError),
        };
    }
}

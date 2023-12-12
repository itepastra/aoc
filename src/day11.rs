use crate::utils::read_lines;
use std::str::FromStr;

pub fn day() {
    dayp1();
    dayp2();
}

fn dayp1() {
    let mut observed_galaxies = Vec::new();
    let mut expandable_cols = [true].repeat(256);
    let mut expandable_rows = Vec::new();
    if let Ok(lines) = read_lines("data/day11.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                expandable_rows.push(ip.chars().all(|p| p == '.'));
                expandable_cols = ip
                    .chars()
                    .zip(expandable_cols.iter())
                    .map(|(f, i)| *i && f == '.')
                    .collect();

                let _: Vec<_> = ip
                    .chars()
                    .enumerate()
                    .map(|(j, f)| match f {
                        '#' => observed_galaxies.push((i, j)),
                        _ => {}
                    })
                    .collect();
            }
        }
    }
    let col_offset: Vec<_> = expandable_cols
        .iter()
        .scan(0, |acc, cur| {
            if *cur {
                *acc += 1;
            }
            return Some(*acc);
        })
        .collect();
    let row_offset: Vec<_> = expandable_rows
        .iter()
        .scan(0, |acc, cur| {
            if *cur {
                *acc += 1;
            }
            return Some(*acc);
        })
        .collect();
    let expanded_galaxies: Vec<(usize, usize)> = observed_galaxies
        .into_iter()
        .map(|(x, y)| (x + row_offset[x], y + col_offset[y]))
        .collect();
    let og_slice = expanded_galaxies.as_slice();
    let mut total = 0;
    for i in 0..og_slice.len() {
        for j in i + 1..og_slice.len() {
            let dist = galaxy_distance(og_slice[i], og_slice[j]);
            total += dist;
        }
    }
    println!("day 11 part 1: {}", total);
}

fn dayp2() {
    let mut observed_galaxies = Vec::new();
    let mut expandable_cols = [true].repeat(256);
    let mut expandable_rows = Vec::new();
    if let Ok(lines) = read_lines("data/day11.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                expandable_rows.push(ip.chars().all(|p| p == '.'));
                expandable_cols = ip
                    .chars()
                    .zip(expandable_cols.iter())
                    .map(|(f, i)| *i && f == '.')
                    .collect();

                let _: Vec<_> = ip
                    .chars()
                    .enumerate()
                    .map(|(j, f)| match f {
                        '#' => observed_galaxies.push((i, j)),
                        _ => {}
                    })
                    .collect();
            }
        }
    }
    let col_offset: Vec<_> = expandable_cols
        .into_iter()
        .scan(0, |acc, cur| {
            if cur {
                *acc += 1000000-1;
            }
            return Some(*acc);
        })
        .collect();
    let row_offset: Vec<_> = expandable_rows
        .into_iter()
        .scan(0, |acc, cur| {
            if cur {
                *acc += 1000000-1;
            }
            return Some(*acc);
        })
        .collect();
    let expanded_galaxies: Vec<(usize, usize)> = observed_galaxies
        .into_iter()
        .map(|(x, y)| (x + row_offset[x], y + col_offset[y]))
        .collect();
    let og_slice = expanded_galaxies.as_slice();
    let mut total = 0;
    for i in 0..og_slice.len() {
        for j in i + 1..og_slice.len() {
            let dist = galaxy_distance(og_slice[i], og_slice[j]);
            total += dist;
        }
    }
    println!("day 11 part 2: {}", total);
}

fn galaxy_distance((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> usize {
    return abs_diff(ax, bx) + abs_diff(ay, by);
}

fn abs_diff(slf: usize, other: usize) -> usize {
    if slf < other {
        other - slf
    } else {
        slf - other
    }
}

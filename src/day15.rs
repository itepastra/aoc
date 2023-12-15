use crate::utils::read_lines;
use std::collections::HashMap;

pub fn day() {
    dayp1();
    dayp2();
}

fn dayp1() {
    let mut total = 0;
    if let Ok(mut lines) = read_lines("data/day15.txt") {
        if let Some(Ok(line)) = lines.next() {
            for part in line.split(",") {
                total += hash(part);
            }
        }
    }
    println!("day 15 part 1: {}", total);
}

fn dayp2() {
    let mut hashmap = vec![vec![]; 256];
    let mut total = 0;
    if let Ok(mut lines) = read_lines("data/day15.txt") {
        if let Some(Ok(line)) = lines.next() {
            for part in line.split(",") {
                if let Some((l, r)) = part.split_once('=') {
                    if let Ok(lensnum) = r.parse::<usize>() {
                        let phash = hash(l);
                        if let Some(waa) = hashmap[phash].iter().position(|(a, _)| *a == l) {
                            hashmap[phash][waa] = (l, lensnum);
                        } else {
                            hashmap[phash].push((l, lensnum));
                        }
                    }
                } else if let Some((l, _)) = part.split_once('-') {
                    let phash = hash(l);
                    if let Some(pos) = hashmap[phash].iter().position(|(a, _)| *a == l) {
                        hashmap[phash].remove(pos);
                    }
                } else {
                    todo!();
                }
            }
            for (i, bx) in hashmap.iter().enumerate() {
                for (j, (_, fp)) in bx.iter().enumerate() {
                    total += (i + 1) * (j + 1) * fp;
                }
            }
        }
    }
    println!("day 15 part 2: {}", total);
}

fn hash(string: &str) -> usize {
    return string.as_bytes().iter().fold(0, |acc, chr| {
        return ((acc + *chr as usize) * 17) % 256;
    });
}

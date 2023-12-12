use crate::utils::read_lines;
use std::iter::{repeat, once};
use std::collections::HashMap;

pub fn day() {
    dayp1();
    dayp2();
}

fn dayp1() {
    let mut total = 0;
    if let Ok(lines) = read_lines("data/day12.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if let Some((map, streaks)) = ip.split_once(" ") {
                    let line_springs = map
                        .chars()
                        .map(|c| match c {
                            '#' => Spring::Broken,
                            '.' => Spring::Working,
                            '?' => Spring::Unknown,
                            _ => todo!(),
                        })
                        .collect();
                    let line_amounts = streaks.split(",").map(|a| a.parse().unwrap()).collect();

                    let mut cache = HashMap::new();
                    //println!("Trying {:?}, {:?} with total {}",map, line_amounts, total);
                    total += get_options(&line_springs, 0, &line_amounts, 0, 0, Spring::Working, &mut cache);
                }
            }
        }
    }
    println!("day 12 part 1: {}", total);
}

fn mknice(line: &Vec<Spring>) -> String {
   let str = line.iter().map(|s| match s {
    Spring::Unknown => '?',
    Spring::Broken => '#',
    Spring::Working => '.',
   }).collect(); 
    return str
}


fn dayp2() {
    let mut total = 0;
    if let Ok(lines) = read_lines("data/day12.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if let Some((map, streaks)) = ip.split_once(" ") {
                    let mut line_springs: Vec<_> = repeat(map
                        .chars()
                        .map(|c| match c {
                            '#' => Spring::Broken,
                            '.' => Spring::Working,
                            '?' => Spring::Unknown,
                            _ => todo!(),
                        }).chain(once(Spring::Unknown))).take(5).flatten()
                        .collect();
                    //Pop the extra "unknown" at the end
                    let _ = line_springs.pop();
                    let line_amounts = repeat(streaks.split(",").map(|a| a.parse().unwrap())).take(5).flatten().collect();

                    let mut cache = HashMap::new();
                    //println!("Trying {:?}, {:?} with total {}", mknice(&line_springs), line_amounts, total);
                    total += get_options(&line_springs, 0, &line_amounts, 0, 0, Spring::Working, &mut cache);
                }
            }
        }
    }
    println!("day 12 part 2: {}", total);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

fn get_options(
    line: &Vec<Spring>,
    pos: usize,
    broken_streaks: &Vec<usize>,
    broken_streak: usize,
    broken_parts_since_last: usize,
    prev: Spring,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    match cache.get(&(pos, broken_streak, broken_parts_since_last)) {
        Some(amt) => { //println!("found in cache for {:?}, {}", line, amt); 
            return *amt;},
        None => {}
    }
    if broken_streak >= broken_streaks.len() && broken_parts_since_last != 0 {
        cache.insert((pos, broken_streak, broken_parts_since_last), 0);
        return 0;
    }
    
    if pos == line.len() {
        if broken_streak < broken_streaks.len() - 1 {
        cache.insert((pos, broken_streak, broken_parts_since_last), 0);
            return 0;
        } else if broken_streak == broken_streaks.len() && broken_parts_since_last == 0 {
        cache.insert((pos, broken_streak, broken_parts_since_last), 1);
            return 1;
        }
        else if broken_streak >= broken_streaks.len() {
        cache.insert((pos, broken_streak, broken_parts_since_last), 0);
            return 0;
        }
        else if broken_streaks[broken_streak] != broken_parts_since_last {
        cache.insert((pos, broken_streak, broken_parts_since_last), 0);
            return 0;
        } else {
        cache.insert((pos, broken_streak, broken_parts_since_last), 1);
            return 1;
        }
    }

    let opts;
    match (line[pos], prev) {
        (Spring::Working, Spring::Working) => {
            opts = get_options(
                line,
                pos + 1,
                broken_streaks,
                broken_streak,
                0,
                Spring::Working,
                cache,
            );
        }
        (Spring::Working, Spring::Broken) => {
            if broken_parts_since_last != broken_streaks[broken_streak] {
        cache.insert((pos, broken_streak, broken_parts_since_last), 0);
                return 0;
            }
            opts = get_options(
                line,
                pos + 1,
                broken_streaks,
                broken_streak + 1,
                0,
                Spring::Working,
                cache,
            );
        }
        (Spring::Broken, Spring::Broken) => {
            opts = get_options(
                line,
                pos + 1,
                broken_streaks,
                broken_streak,
                broken_parts_since_last + 1,
                Spring::Broken,
                cache,
            );
        }
        (Spring::Broken, Spring::Working) => {
            opts = get_options(
                line,
                pos + 1,
                broken_streaks,
                broken_streak,
                1, // is the first one since it started
                Spring::Broken,
                cache,
            );
        }
        (Spring::Unknown, Spring::Broken) => {
            let mut l1 = line.clone();
            l1[pos] = Spring::Broken;
            let mut l2 = line.clone();
            l2[pos] = Spring::Working;
            if  broken_streak < broken_streaks.len() && broken_parts_since_last != broken_streaks[broken_streak] {
                opts = get_options(&l1, pos + 1, broken_streaks, broken_streak, broken_parts_since_last + 1, Spring::Broken, cache,);
            } else {
            opts = 
                // unknown == broken
                get_options(&l1, pos + 1, broken_streaks, broken_streak, broken_parts_since_last + 1, Spring::Broken,cache,) + 
                // unknown == working
                get_options(&l2, pos + 1, broken_streaks, broken_streak + 1, 0, Spring::Working,cache,);
            }
        }
        (Spring::Unknown, Spring::Working) => {
            let mut l1 = line.clone();
            l1[pos] = Spring::Broken;
            let mut l2 = line.clone();
            l2[pos] = Spring::Working;
            opts = 
                // unknown == broken
                get_options(&l1, pos + 1, broken_streaks, broken_streak, 1, Spring::Broken,cache,) + 
                // unknown == working
                get_options(&l2, pos + 1, broken_streaks, broken_streak, 0, Spring::Working,cache,);
        }
        _ => {
            todo!()
        }
    }
    cache.insert((pos, broken_streak, broken_parts_since_last), opts);

    return opts;
}


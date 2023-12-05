use std::{fs::File, io::BufReader};

use crate::utils::read_lines;

pub fn day() {
    dayp1();
    dayp2();
}

#[derive(Debug, Clone)]
struct Range {
    from: u64,
    to: u64,
}

fn dayp1() {
    if let Ok(mut lines) = read_lines("data/day5.txt") {
        let waa: Vec<u64>;
        if let Some(Ok(fst)) = lines.next() {
            waa = fst
                .strip_prefix("seeds: ")
                .unwrap_or("")
                .split(" ")
                .map(|s| s.parse().unwrap_or(0))
                .collect();
        } else {
            waa = Vec::new();
        }

        let soils = do_block(&mut lines, waa);
        let fertilizers = do_block(&mut lines, soils);
        let waters = do_block(&mut lines, fertilizers);
        let lights = do_block(&mut lines, waters);
        let temperatures = do_block(&mut lines, lights);
        let humidities = do_block(&mut lines, temperatures);
        let locations = do_block(&mut lines, humidities);
        println!("day 5 part 1: {}", locations.iter().min().unwrap());
    }
}

fn dayp2() {
    if let Ok(mut lines) = read_lines("data/day5.txt") {
        let waa: Vec<u64>;
        if let Some(Ok(fst)) = lines.next() {
            waa = fst
                .strip_prefix("seeds: ")
                .unwrap_or("")
                .split(" ")
                .map(|s| s.parse().unwrap_or(0))
                .collect();
        } else {
            waa = Vec::new();
        }
        let mut seeds = Vec::new();
        let mut is_range = false;
        let mut start = 0;
        for val in waa.iter() {
            if is_range {
                seeds.push(Range {
                    from: start,
                    to: start + val,
                });
                is_range = false;
            } else {
                start = *val;
                is_range = true;
            }
        }

        let soils = do_block_range(&mut lines, seeds);
        let fertilizers = do_block_range(&mut lines, soils);
        let waters = do_block_range(&mut lines, fertilizers);
        let lights = do_block_range(&mut lines, waters);
        let temperatures = do_block_range(&mut lines, lights);
        let humidities = do_block_range(&mut lines, temperatures);
        let locations = do_block_range(&mut lines, humidities);
        println!(
            "day 5 part 2: {}",
            locations.iter().map(|r| r.from).min().unwrap()
        );
    }
}

fn do_range_map(map_vec: &Vec<(u64, u64, u64)>, val: &Range, outvec: &mut Vec<Range>) {
    let mut still_range = Some(Range {
        from: val.from,
        to: val.to,
    });
    for (dest, src, len) in map_vec.iter() {
        if let Some(sr) = &still_range {
            let end = src + len;
            match (sr.from >= *src, sr.from < end, sr.to > *src, sr.to <= end) {
                (true, true, true, true) => {
                    let new_range = Range {
                        from: sr.from - src + dest,
                        to: sr.to - src + dest,
                    };
                    still_range = None;
                    outvec.push(new_range);
                }
                (true, false, true, false) => {} // fully to the right
                (false, true, false, true) => {} // fully to the left
                (true, true, true, false) => {
                    let new_range = Range {
                        from: sr.from - src + dest,
                        to: dest + len,
                    };
                    still_range = Some(Range {
                        from: end,
                        to: sr.to,
                    });
                    outvec.push(new_range);
                }
                (false, true, true, true) => {
                    let new_range = Range {
                        from: *dest,
                        to: dest + sr.to - src,
                    };
                    still_range = Some(Range {
                        from: sr.from,
                        to: *src,
                    });
                    outvec.push(new_range);
                }
                (false, true, true, false) => {
                    let new_range = Range {
                        from: *dest,
                        to: dest + len,
                    };
                    outvec.push(new_range);
                    let left_range = Range {
                        from: sr.from,
                        to: *src,
                    };
                    do_range_map(map_vec, &left_range, outvec);
                    let right_range = Range {
                        from: end,
                        to: sr.to,
                    };
                    do_range_map(map_vec, &right_range, outvec);
                    still_range = None;
                }
                (_,_, false, false) // end of range is greater and smaller than the end of the
                                    // target at the same time
                | (false, false, _, _) // start of range is greater and smaller than the end of the
                                       // target at the same time
                | (true, _, _, true) // the start is larger, end is smaller but middle is not in
                                     // the range
                 => {
                    println!(
                        "Impossible situation encountered: {:?}, {}, {} (dest: {})",
                        sr, src, len, dest
                    );
                    todo!()
                }
            };
        }
    }
    if let Some(sr) = still_range {
        outvec.push(sr);
    }
}

fn do_block_range(lines: &mut std::io::Lines<BufReader<File>>, in_vec: Vec<Range>) -> Vec<Range> {
    lines
        .skip_while(|p| p.as_ref().is_ok_and(|f| f.is_empty()))
        .next();
    let mut outvec = Vec::new();
    let mut map_vec = Vec::new();
    for ln in lines.take_while(|p| p.as_ref().is_ok_and(|f| !f.is_empty())) {
        //line is every line with a rule
        if let Ok(line) = ln {
            map_vec.push(parse_line(line).unwrap());
        }
    }

    for val in in_vec.iter() {
        do_range_map(&map_vec, val, &mut outvec);
    }
    return outvec;
}

fn do_block(lines: &mut std::io::Lines<BufReader<File>>, in_vec: Vec<u64>) -> Vec<u64> {
    lines
        .skip_while(|p| p.as_ref().is_ok_and(|f| f.is_empty()))
        .next();
    let mut outvec = Vec::new();
    for val in in_vec.iter() {
        outvec.push(*val);
    }
    for ln in lines.take_while(|p| p.as_ref().is_ok_and(|f| !f.is_empty())) {
        if let Ok(line) = ln {
            let (dest, src, len) = parse_line(line).unwrap();
            for (i, val) in in_vec.iter().enumerate() {
                if val >= &src && val < &(src + len) {
                    outvec[i] = dest + val - src;
                }
            }
        }
    }
    return outvec;
}

fn parse_line(line: String) -> Option<(u64, u64, u64)> {
    if let Some((a, bc)) = line.split_once(" ") {
        if let Some((b, c)) = bc.split_once(" ") {
            if let (Ok(x), Ok(y), Ok(z)) = (a.parse(), b.parse(), c.parse()) {
                return Some((x, y, z));
            }
        }
    }
    return None;
}

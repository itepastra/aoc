use core::f64;

use crate::utils::read_lines;
pub fn day() {
    dayp1();
    dayp2();
}

fn dayp1() {
    if let Ok(mut lines) = read_lines("data/day6.txt") {
        if let Some(Ok(time_line)) = lines.next() {
            if let Some(Ok(win_line)) = lines.next() {
                let times: Vec<i64> = time_line
                    .split(" ")
                    .filter(|p| p != &"")
                    .map(|f| f.parse())
                    .filter(|p| p.is_ok())
                    .map(|f| f.unwrap())
                    .collect();
                let distances: Vec<i64> = win_line
                    .split(" ")
                    .filter(|p| p != &"")
                    .map(|f| f.parse())
                    .filter(|p| p.is_ok())
                    .map(|f| f.unwrap())
                    .collect();
                let races: Vec<Race> = times
                    .iter()
                    .zip(distances.iter())
                    .map(|(t, d)| Race {
                        time: *t,
                        record: *d,
                    })
                    .collect();
                let bounds: Vec<(i64, i64)> = races.iter().map(|f| win_bounds(f)).collect();
                let win_opts: Vec<i64> = bounds.iter().map(|(l, u)| u - l + 1).collect();
                println!("day 6 part 1: {}", win_opts.iter().fold(1, |a, b| a * b));
            }
        }
    }
}

fn dayp2() {
    if let Ok(mut lines) = read_lines("data/day6.txt") {
        if let Some(Ok(time_line)) = lines.next() {
            if let Some(Ok(win_line)) = lines.next() {
                let times: Vec<i64> = time_line
                    .split(" ")
                    .filter(|p| p != &"")
                    .map(|f| f.parse())
                    .filter(|p| p.is_ok())
                    .map(|f| f.unwrap()) // this is safe since all are OK due to the filter above
                    .collect();
                let distances: Vec<i64> = win_line
                    .split(" ")
                    .filter(|p| p != &"")
                    .map(|f| f.parse())
                    .filter(|p| p.is_ok()) // this is safe since all are OK due to the filter above
                    .map(|f| f.unwrap())
                    .collect();
                let t2 = concat_new(times.as_slice());
                let d2 = concat_new(distances.as_slice());
                let race = Race {
                    time: t2,
                    record: d2,
                };
                let (lower, upper) = win_bounds(&race);
                println!("day 6 part 2: {}", upper - lower + 1);
            }
        }
    }
}

#[derive(Debug)]
struct Race {
    time: i64,
    record: i64,
}

fn concat_new(vec: &[i64]) -> i64 {
    let t = vec
        .iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string());
    t.parse::<i64>().unwrap()
}

fn win_bounds(race: &Race) -> (i64, i64) {
    let time = race.time as f64;
    let rec = race.record as f64 + 0.1;
    let h1 = (time - (time * time - 4.0 * rec).sqrt()) / 2.0;
    let h2 = (time + (time * time - 4.0 * rec).sqrt()) / 2.0;
    return (f64::ceil(h1) as i64, (f64::floor(h2)) as i64);
}

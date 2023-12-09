use crate::utils::read_lines;

pub fn day() {
    dayp1();
}

fn dayp1() {
    if let Ok(lines) = read_lines("data/day9.txt") {
        let mut total_next = 0;
        let mut total_prev = 0;
        for line in lines {
            if let Ok(ip) = line {
                let nums: Vec<i64> = ip
                    .split(" ")
                    .filter(|p| p != &"")
                    .map(|f| f.parse().unwrap())
                    .collect();
                if nums.len() > 0 {
                    let next = calculate_next(&nums);
                    let prev = calculate_previous(&nums);
                    total_next += next;
                    total_prev += prev;
                }
            }
        }
        println!("day 9 part 1: {}", total_next);
        println!("day 9 part 2: {}", total_prev);
    }
}

fn calculate_next(nums: &Vec<i64>) -> i64 {
    let diffs: Vec<i64> = nums
        .iter()
        .scan(0, |acc, c| {
            let n = *c - *acc;
            *acc = *c;
            return Some(n);
        })
        .skip(1)
        .collect();
    if diffs.iter().all(|p| *p == 0) {
        return *nums.last().unwrap_or(&i64::MAX);
    } else {
        let next_in = calculate_next(&diffs);
        let var_name = nums.last().unwrap_or(&0) + next_in;
        return var_name;
    }
}

fn calculate_previous(nums: &Vec<i64>) -> i64 {
    let diffs: Vec<i64> = nums
        .iter()
        .scan(0, |acc, c| {
            let n = *c - *acc;
            *acc = *c;
            return Some(n);
        })
        .skip(1)
        .collect();
    if diffs.iter().all(|p| *p == 0) {
        return *nums.first().unwrap_or(&i64::MAX);
    } else {
        let prev_in = calculate_previous(&diffs);
        let var_name = nums.first().unwrap_or(&0) - prev_in;
        return var_name;
    }
}

use crate::utils::{read_lines, frog};

pub fn day() {
    if let Ok(lines) = read_lines("data/day1.txt") {
        let mut total = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut first: Option<char> = None;
                let mut last: char = '0';
                for char in ip.chars() {
                    if char.is_numeric() {
                        if first == None {
                            first = Some(char);
                        }
                        last = char;
                    }
                }
                if let Some(f) = first {
                    total += format!("{}{}", f, last).parse::<i32>().unwrap();
                }
            }
        }
        println!("day 1 part 1: {}", total);
    }
    if let Ok(lines) = read_lines("data/day1.txt") {
        let mut total = 0;
        for line in lines {
            if let Ok(ip) = line {
                if let Some((f, l)) = frog(ip.as_bytes(), None, None) {
                    total += 10 * f + l;
                    //println!("{} gives {}-{}, total: {}", ip, f, l, total);
                }
            }
        }
        println!("day 1 part 2: {}", total);
    }
}

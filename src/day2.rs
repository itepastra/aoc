use crate::utils::read_lines;
use crate::max;

pub fn day() {
    let redtotal = 12;
    let greentotal = 13;
    let bluetotal = 14;
    if let Ok(lines) = read_lines("data/day2.txt") {
        let mut total = 0;
        let mut total2 = 0;
        for line in lines {
            if let Ok(ip) = line {
                if let Some(r) = ip.as_str().strip_prefix("Game ") {
                    if let Some((f, s)) = r.split_once(": ") {
                        let mut min_red = 0;
                        let mut min_blue = 0;
                        let mut min_green = 0;
                        for round in s.split("; ") {
                            for color in round.split(", ").map(|f| {
                                return f.split_once(' ');
                            }) {
                                match color {
                                    Some((n, "blue")) => {
                                        min_blue = max(min_blue, n.parse().unwrap_or(0));
                                    }
                                    Some((n, "red")) => {
                                        min_red = max(min_red, n.parse().unwrap_or(0));
                                    }
                                    Some((n, "green")) => {
                                        min_green = max(min_green, n.parse().unwrap_or(0));
                                    }
                                    _ => {println!("{:?}", color)}
                                }
                            }
                        }
                        total2 += min_red * min_green * min_blue;
                        if bluetotal >= min_blue && greentotal >= min_green && redtotal >= min_red {
                            total += f.parse().unwrap_or(0);
                        }
                    }
                }
            }
        }
        println!("day 2 part 1: {}", total);
        println!("day 2 part 2: {}", total2);
    }
}

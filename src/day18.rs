use crate::utils::read_lines;

pub fn day() {
    let mut surf = 0;
    let mut x = 0;
    let mut y = 0;
    let mut surf2 = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut exterior = 0;
    let mut exterior2 = 0;
    if let Ok(lines) = read_lines("data/day18.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let chars: Vec<_> = ip.splitn(3, ' ').collect();
                let dir = direction(chars[0]);
                let amt: i32 = chars[1].parse().unwrap();
                let dist = i64::from_str_radix(&chars[2][2..7], 16).unwrap_or(0);
                let dir2 = i64::from_str_radix(&chars[2][7..8], 16);
                match dir {
                    Direction::Right => {
                        surf += x * y - (x + amt) * y;
                        x += amt;
                    }
                    Direction::Left => {
                        surf += x * y - (x - amt) * y;
                        x -= amt;
                    }
                    Direction::Down => {
                        surf += x * (y + amt) - x * y;
                        y += amt;
                    }
                    Direction::Up => {
                        surf += x * (y - amt) - x * y;
                        y -= amt;
                    }
                }
                match dir2 {
                    Ok(0) => {
                        surf2 += x2 * y2  - (x2 + dist) * y2;
                        x2 += dist;
                    }
                    Ok(1) => {
                        surf2 += x2 * (y2 + dist) - x2 * y2;
                        y2 += dist;
                    }
                    Ok(2) => {
                        surf2 += x2 * y2  - (x2 - dist) * y2;
                        x2 -= dist;
                    }
                    Ok(3) => {
                        surf2 += x2 * (y2 - dist) - x2 * y2;
                        y2 -= dist;
                    }
                    _ => todo!(),
                }
                exterior += amt;
                exterior2 += dist;
            }
        }
        surf += y * -x;
        println!("day 18 part 1: {}", surf / 2 + 1 + exterior / 2);
        println!("day 18 part 2: {}", (surf2 + y2 * -x2) / 2 + 1 + exterior2 / 2);
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn direction(sr: &str) -> Direction {
    match sr {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => todo!(),
    }
}

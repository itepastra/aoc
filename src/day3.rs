use crate::utils::read_lines;

pub fn day() {
    dayp1();
    dayp2();
}

fn dayp1() {
    let mut ls: Vec<Vec<char>> = Vec::new();
    let mut total = 0;
    if let Ok(lines) = read_lines("data/day3.txt") {
        for line in lines {
            if let Ok(ip) = line {
                ls.append(&mut vec![ip.chars().collect()]);
            }
        }
    }
    for (idx, line) in ls.iter().enumerate() {
        let mut skip = 0;
        for (ichar, char) in line.iter().enumerate() {
            if char.is_numeric() && skip <= 0 {
                let part;
                let num;
                (part, skip, num) = check_part(&ls, idx, ichar);
                if part {
                    total += num
                }
            } else if skip > 0 {
                skip -= 1;
            }
        }
    }
    println!("day 3 part 1: {}", total)
}

fn dayp2() {
    let mut ls: Vec<Vec<char>> = Vec::new();
    let mut total = 0;
    if let Ok(lines) = read_lines("data/day3.txt") {
        for line in lines {
            if let Ok(ip) = line {
                ls.append(&mut vec![ip.chars().collect()]);
            }
        }
    }
    for (idx, line) in ls.iter().enumerate() {
        for (ichar, char) in line.iter().enumerate() {
            if char == &'*' {
                let (is_gear, num1, num2) = check_gear(&ls, idx, ichar);
                if is_gear {
                    total += num1 * num2
                }
            }
        }
    }
    println!("day 3 part 2: {}", total)
}

fn check_gear(chars: &Vec<Vec<char>>, line_idx: usize, char_idx: usize) -> (bool, i32, i32) {
    let around = iter_around(line_idx, char_idx, 1, chars.len(), chars[line_idx].len());
    let mut num_locations: [Option<i32>; 9] =
        [None, None, None, None, None, None, None, None, None];
    let mut n1 = 0;
    let mut n2 = 0;
    for (i, (y, x)) in around.enumerate() {
        let c = chars[y][x];
        if c.is_numeric() {
            num_locations[i] = Some(get_full_num(&chars, y, x));
            for n in num_locations {
                match n {
                    Some(a) if a != n1 && a != n2 => {
                        n2 = n1;
                        n1 = a;
                    },
                    _ => {}
                }
            }
        }
    }

    return (true, n1, n2);
}

fn get_full_num(chars: &Vec<Vec<char>>, line_idx: usize, char_idx: usize) -> i32 {
    let line = &chars[line_idx];

    let mut begin = line
        .iter()
        .rev()
        .skip(line.len() - char_idx)
        .take_while(|p| p.is_numeric())
        .collect::<Vec<&char>>();
    begin.reverse();
    let mut end = line
        .iter()
        .skip(char_idx)
        .take_while(|p| p.is_numeric())
        .collect::<Vec<&char>>();
    begin.append(&mut end);
    return begin.into_iter().collect::<String>().parse().unwrap();
}

fn check_part(chars: &Vec<Vec<char>>, line_idx: usize, char_idx: usize) -> (bool, usize, i32) {
    let line = chars[line_idx].clone();
    let num: Vec<&char> = line
        .iter()
        .skip(char_idx)
        .take_while(|f| f.is_numeric())
        .collect();
    let n: String = num.clone().into_iter().collect();
    let around = iter_around(line_idx, char_idx, num.len(), chars.len(), line.len());
    for (y, x) in around {
        let c = chars[y][x];
        if c != '.' && !c.is_numeric() {
            return (true, num.len(), n.parse().unwrap());
        }
    }
    return (false, num.len(), n.parse().unwrap());
}

enum Side {
    End,
    Top,
    Bottom,
    Middle,
}

struct AroundPart {
    line_idx: usize,
    char_idx: usize,
    len: usize,
    table_x: usize,
    table_y: usize,
    side: Side,
    pos: usize,
}

fn iter_around(
    line_idx: usize,
    char_idx: usize,
    len: usize,
    table_x: usize,
    table_y: usize,
) -> AroundPart {
    return AroundPart {
        line_idx,
        char_idx,
        len,
        table_x,
        table_y,
        side: Side::Top,
        pos: 0,
    };
}

impl Iterator for AroundPart {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.side {
            Side::Top => {
                if self.line_idx == 0 {
                    self.side = Side::Middle;
                    return self.next();
                }
                if self.char_idx == 0 && self.pos == 0 {
                    self.pos = 1;
                    return self.next();
                }
                let ci = self.char_idx + self.pos - 1;
                self.pos += 1;
                if self.pos > self.len + 1 {
                    self.side = Side::Middle;
                    self.pos = 0;
                    if self.char_idx + self.len == self.table_x - 1 {
                        return self.next();
                    }
                }
                return Some((self.line_idx - 1, ci));
            }
            Side::Middle => {
                if self.char_idx == 0 && self.pos == 0 {
                    self.pos = 1;
                    return self.next();
                }
                let ci = self.char_idx + self.pos - 1;
                self.pos += 1;
                if self.pos > self.len + 1 {
                    self.pos = 0;
                    self.side = Side::Bottom;
                    if self.char_idx + self.len == self.table_x - 1 {
                        return self.next();
                    }
                }
                return Some((self.line_idx, ci));
            }
            Side::Bottom => {
                if self.line_idx + 1 >= self.table_y {
                    self.side = Side::End;
                    self.pos = 0;
                    return self.next();
                }
                if self.char_idx == 0 && self.pos == 0 {
                    self.pos = 1;
                    return self.next();
                }
                let ci = self.char_idx + self.pos - 1;
                self.pos += 1;
                if self.pos > self.len + 1 {
                    self.pos = 0;
                    self.side = Side::End;
                    if self.char_idx + self.len == self.table_x - 1 {
                        return self.next();
                    }
                }
                return Some((self.line_idx + 1, ci));
            }
            Side::End => {
                return None;
            }
        }
    }
}

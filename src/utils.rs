use std::{path::Path, fs::File, io::{self, BufRead}};

pub fn frog(s: &[u8], first: Option<i32>, last: Option<i32>) -> Option<(i32, i32)> {
    match s {
        [b'z', b'e', b'r', b'o', ..] | [b'0', ..] => {
            frog(&s[1..], Some(first.unwrap_or(0)), Some(0))
        }
        [b'o', b'n', b'e', ..] | [b'1', ..] => frog(&s[1..], Some(first.unwrap_or(1)), Some(1)),
        [b't', b'w', b'o', ..] | [b'2', ..] => frog(&s[1..], Some(first.unwrap_or(2)), Some(2)),
        [b't', b'h', b'r', b'e', b'e', ..] | [b'3', ..] => {
            frog(&s[1..], Some(first.unwrap_or(3)), Some(3))
        }
        [b'f', b'o', b'u', b'r', ..] | [b'4', ..] => {
            frog(&s[1..], Some(first.unwrap_or(4)), Some(4))
        }
        [b'f', b'i', b'v', b'e', ..] | [b'5', ..] => {
            frog(&s[1..], Some(first.unwrap_or(5)), Some(5))
        }
        [b's', b'i', b'x', ..] | [b'6', ..] => frog(&s[1..], Some(first.unwrap_or(6)), Some(6)),
        [b's', b'e', b'v', b'e', b'n', ..] | [b'7', ..] => {
            frog(&s[1..], Some(first.unwrap_or(7)), Some(7))
        }
        [b'e', b'i', b'g', b'h', b't', ..] | [b'8', ..] => {
            frog(&s[1..], Some(first.unwrap_or(8)), Some(8))
        }
        [b'n', b'i', b'n', b'e', ..] | [b'9', ..] => {
            frog(&s[1..], Some(first.unwrap_or(9)), Some(9))
        }
        [_, rest @ ..] => return frog(rest, first, last),
        [] => return Some((first?, last?)),
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

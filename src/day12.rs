use crate::utils::read_lines;

pub fn day() {
    dayp1();
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

                    println!("Trying {:?}, {:?} with total {}", map, line_amounts, total);
                    total += get_options(&line_springs, 0, &line_amounts, 0, 0, Spring::Working);
                }
            }
        }
    }
    println!("day 12 part 1: {}", total);
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
) -> usize {
    println!("{:?}, at pos {}, at streak {},with n in streak {}", line, pos,  broken_streak, broken_parts_since_last);
    if broken_streak >= broken_streaks.len() && broken_parts_since_last != 0 {
        println!("too many streaks... F {:?}", line);
        return 0;
    }
    
    if pos == line.len() {
        if broken_streak < broken_streaks.len() - 1 {
            println!("Not all streaks were done");
            return 0;
        } else if broken_streak == broken_streaks.len() && broken_parts_since_last == 0 {
            println!("still allowed, since it's cleann {:?}", line);
            return 1;
        }
         else if broken_streak >= broken_streaks.len() {
            println!("too many streaks... F {:?}", line);
            return 0;
        }
        else if broken_streaks[broken_streak] != broken_parts_since_last {
            println!("Something was not allowed about {:?}, {:?}", line, broken_streaks);
            return 0;
        } else {
            println!("I think this is allowed {:?}", line);
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
            );
        }
        (Spring::Working, Spring::Broken) => {
            if broken_parts_since_last != broken_streaks[broken_streak] {
                println!("{:?}'s streak {} was incorrect", line, broken_streak);
                return 0;
            }
            opts = get_options(
                line,
                pos + 1,
                broken_streaks,
                broken_streak + 1,
                0,
                Spring::Working,
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
            );
        }
        (Spring::Unknown, Spring::Broken) => {
            let mut l1 = line.clone();
            l1[pos] = Spring::Broken;
            let mut l2 = line.clone();
            l2[pos] = Spring::Working;
            if  broken_streak < broken_streaks.len() && broken_parts_since_last != broken_streaks[broken_streak] {
                println!("{:?} was incorrect, trying only {:?}", l2, l1);
                return get_options(&l1, pos + 1, broken_streaks, broken_streak, broken_parts_since_last + 1, Spring::Broken);
            }
            opts = 
                // unknown == broken
                get_options(&l1, pos + 1, broken_streaks, broken_streak, broken_parts_since_last + 1, Spring::Broken) + 
                // unknown == working
                get_options(&l2, pos + 1, broken_streaks, broken_streak + 1, 0, Spring::Working);
        }
        (Spring::Unknown, Spring::Working) => {
            let mut l1 = line.clone();
            l1[pos] = Spring::Broken;
            let mut l2 = line.clone();
            l2[pos] = Spring::Working;
            opts = 
                // unknown == broken
                get_options(&l1, pos + 1, broken_streaks, broken_streak, 1, Spring::Broken) + 
                // unknown == working
                get_options(&l2, pos + 1, broken_streaks, broken_streak, 0, Spring::Working);
        }
        _ => {
            todo!()
        }
    }

    return opts;
}


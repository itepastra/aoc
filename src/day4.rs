use crate::utils::read_lines;

pub fn day() {
    dayp1();
    dayp2();
}

fn dayp1() {
    if let Ok(lines) = read_lines("data/day4.txt") {
        let mut total = 0;
        for line in lines {
            if let Ok(ip) = line {
                //println!("{}", ip);
                if let Some(r) = ip.as_str().strip_prefix("Card ") {
                    if let Some((_, s)) = r.split_once(": ") {
                        if let Some((win, mine)) = s.split_once(" | ") {
                            let spps = win.split(" ").filter(|p| p != &"");
                            let ppps = mine.split(" ").filter(|p| p != &"");
                            // println!(
                            //     "{:?}, {:?}",
                            //     spps.clone().collect::<Vec<_>>(),
                            //     ppps.clone().collect::<Vec<_>>()
                            // );
                            let mut points = None;
                            for ele in ppps {
                                for chk in spps.clone() {
                                    if ele == chk {
                                        match points {
                                            Some(p) => points = Some(p * 2),
                                            None => points = Some(1),
                                        }
                                    }
                                }
                            }
                            //println!("{:?}", points);
                            total += points.unwrap_or(0);
                        }
                    }
                }
            }
        }
        println!("day 4 part 1: {}", total)
    }
}

#[derive(Debug)]
struct Card {
    number:usize,
    winning:Vec<usize>,
    own:Vec<usize>,
    amount:usize,
}

fn win_amt(c: &Card) -> usize {
    let mut win_amt = 0;
    for val in c.own.iter() {
        for ele in c.winning.iter() {
            if ele == val {
                win_amt += 1;
            }
        }
    }
    return win_amt;
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.number.eq(&other.number);
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.number.partial_cmp(&other.number);
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.number.cmp(&other.number);
    }
}

fn dayp2() {
    if let Ok(lines) = read_lines("data/day4.txt") {
        let mut cards: Vec<Card> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                if let Some(r) = ip.as_str().strip_prefix("Card") {
                    if let Some((n, s)) = r.split_once(": ") {
                        if let Some((win, mine)) = s.split_once(" | ") {
                            let spps = win.split(" ").filter(|p| p != &"");
                            let ppps = mine.split(" ").filter(|p| p != &"");

                            let wv = spps.map(|f| f.parse().unwrap_or(0)).collect();
                            let mv = ppps.map(|f| f.parse().unwrap_or(0)).collect();

                            let c = Card {
                                number: n.trim().parse().unwrap_or(0),
                                winning: wv,
                                own: mv,
                                amount: 1,
                            };

                            cards.push(c);
                        }
                    }
                }
            }
        }
        let mut total = 0;
        for cidx in 0..cards.len() {
            let cc = &cards[cidx];
            let cn = cc.number;
            let ca = cc.amount;
            total += cc.amount;
            let w = win_amt(cc);
            if w > 0 {
                println!("card {} won: {} tickets on {} cards", cn, w, ca);
                for off in 0..w {
                    println!("adding {} to {}", ca, cidx+off+1);
                    let co = &mut (cards[cidx + off+1]);
                    co.amount = co.amount + ca;
                }
            }
        }
        println!("day 4 part 2: {}", total);
    }
}

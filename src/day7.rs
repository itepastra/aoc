use std::{
    cmp::Ordering::{self, Greater, Less},
    mem::MaybeUninit,
    str::FromStr,
};

use crate::utils::read_lines;

pub fn day() {
    dayp2();
}

fn dayp2() {
    if let Ok(lines) = read_lines("data/day7.txt") {
        let mut hands = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                if let Some((cards, bet_str)) = ip.split_once(" ") {
                    let mut card_value_iter = cards.chars().map(|c| c.to_string().parse().unwrap());
                    let mut card_values: [CardValue; 5] =
                        unsafe { MaybeUninit::uninit().assume_init() };
                    for i in 0..5 {
                        card_values[i] = card_value_iter.next().unwrap();
                    }
                    let bet = bet_str.trim().parse().unwrap();
                    let mut card_map = card_values.iter().fold(Vec::new(), |acc, v| {
                        let mut new_acc = Vec::new();
                        let mut found = false;
                        for (cnt, el) in acc {
                            if el == *v {
                                new_acc.push((cnt + 1, *v));
                                found = true;
                            } else {
                                new_acc.push((cnt, el));
                            }
                        }
                        if !found {
                            new_acc.push((1, *v));
                        }
                        return new_acc;
                    });
                    card_map.sort_unstable_by(|a, b| b.cmp(a));
                    let hand_type1 = match card_map.as_slice() {
                        [(5,_)] => HandType::FiveOfAKind,
                        [(4,_),..] => HandType::FourOfAKind,
                        [(3,_), (2,_)] => HandType::FullHouse,
                        [(3,_),..] => HandType::ThreeOfAKind,
                        [(2,_), (2,_),..] => HandType::TwoPair,
                        [(2,_),..] => HandType::Pair,
                        [(1,_),..] => HandType::High,
                        _ => todo!(),
                    };
                    let hand_type = match card_map.as_slice() {
                        [(5, _)]
                        | [(4, _), (1, CardValue::Jack)]
                        | [(3, _), (2, CardValue::Jack)]
                        | [(3, CardValue::Jack), (2, _)]
                        | [(4, CardValue::Jack), (1, _)] => HandType::FiveOfAKind,
                        [(4, _), (1, _)]
                        | [(3, _),(1,_), (1, CardValue::Jack)]
                        | [(2, _), (2, CardValue::Jack), (1, _)]
                        | [(3, CardValue::Jack), (1, _), (1, _)] => HandType::FourOfAKind,
                        [(3, _), (2, _)] | [(2,_), (2,_), (1,CardValue::Jack)] => HandType::FullHouse,
                        [(3, _), ..]
                        | [(2, _),.., (1, CardValue::Jack)]
                        | [(2, CardValue::Jack), (1, _), ..] => HandType::ThreeOfAKind,
                        [(2, _), (2, _), ..] => HandType::TwoPair,
                        [(2, _), ..] | [..,(1, CardValue::Jack)] => HandType::Pair,
                        [(1, _), ..] => HandType::High,
                        _ => todo!(),
                    };
                    let toadd = Hand {
                        bet,
                        hand: card_values,
                        hand_type,
                    };
                    hands.push(toadd);
                }
            }
        }
        hands.sort_unstable();
        let total = hands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, h)| acc + h.bet * (i + 1));
        println!("day 7 part 2: {}", total);
    }
}

#[derive(Debug)]
struct Hand {
    bet: usize,
    hand: [CardValue; 5],
    hand_type: HandType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardValue {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq)]
struct CardValueError;

impl FromStr for CardValue {
    type Err = CardValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => return Ok(CardValue::Two),
            "3" => return Ok(CardValue::Three),
            "4" => return Ok(CardValue::Four),
            "5" => return Ok(CardValue::Five),
            "6" => return Ok(CardValue::Six),
            "7" => return Ok(CardValue::Seven),
            "8" => return Ok(CardValue::Eight),
            "9" => return Ok(CardValue::Nine),
            "T" => return Ok(CardValue::Ten),
            "J" => return Ok(CardValue::Jack),
            "Q" => return Ok(CardValue::Queen),
            "K" => return Ok(CardValue::King),
            "A" => return Ok(CardValue::Ace),
            _ => return Err(CardValueError),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.hand == other.hand;
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let tcomp = self.hand_type.partial_cmp(&other.hand_type);
        match tcomp {
            Some(a) => match a {
                Ordering::Equal => return self.hand.partial_cmp(&other.hand),
                Less => return Some(Less),
                Greater => return Some(Greater),
            },
            None => return None,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.hand.cmp(&other.hand);
    }
}

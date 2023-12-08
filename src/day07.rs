use std::cmp::Ordering;

use itertools::Itertools;

type Input<'a> = Vec<(&'a str, usize)>;

#[derive(Debug)]
pub struct Hand<'a> {
    hand: &'a str,
    bid: usize,
    hand_type: HandType,
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (hand, bid.parse().unwrap())
        })
        .collect_vec()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|each| Hand {
            hand: each.0,
            bid: each.1,
            hand_type: hand_type(each.0),
        })
        .sorted_by(|a, b| {
            if a.hand_type == b.hand_type {
                for card in 0..5 {
                    let a_card = card_value(a.hand.chars().nth(card).unwrap());
                    let b_card = card_value(b.hand.chars().nth(card).unwrap());
                    if a_card != b_card {
                        return a_card.cmp(&b_card);
                    }
                }
                Ordering::Equal
            } else {
                a.hand_type.cmp(&b.hand_type)
            }
        })
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum()
}

// pub fn part2(input: &Input) -> usize {
//
// }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn hand_type(hand: &str) -> HandType {
    if hand.chars().all_equal() {
        return HandType::FiveKind;
    }

    let counts = hand.chars().counts();
    if counts.values().any(|count| count == &4) {
        return HandType::FourKind;
    }
    if counts.values().any(|count| count == &3) && counts.values().any(|count| count == &2) {
        return HandType::FullHouse;
    }
    if counts.values().any(|count| count == &3) {
        return HandType::ThreeKind;
    }
    if counts.values().filter(|count| count == &&2).count() == 2 {
        return HandType::TwoPair;
    }
    if counts.values().any(|count| count == &2) {
        return HandType::OnePair;
    }

    if hand.chars().all_unique() {
        return HandType::HighCard;
    }
    panic!("Unknown hand type")
}

fn card_value(hand: char) -> u32 {
    hand.to_digit(10).unwrap_or_else(|| match hand {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unknown card value")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 6440);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 71503);
    // }
}

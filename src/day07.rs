use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

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
    let joker = false;

    input
        .iter()
        .map(|each| Hand {
            hand: each.0,
            bid: each.1,
            hand_type: hand_type(each.0, false),
        })
        .sorted_by(|a, b| {
            if a.hand_type == b.hand_type {
                for card in 0..5 {
                    let a_card = card_value(a.hand.chars().nth(card).unwrap(), joker);
                    let b_card = card_value(b.hand.chars().nth(card).unwrap(), joker);
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

pub fn part2(input: &Input) -> usize {
    let joker = true;

    input
        .iter()
        .map(|each| Hand {
            hand: each.0,
            bid: each.1,
            hand_type: hand_type(each.0, true),
        })
        .sorted_by(|a, b| {
            if a.hand_type == b.hand_type {
                for card in 0..5 {
                    let a_card = card_value(a.hand.chars().nth(card).unwrap(), joker);
                    let b_card = card_value(b.hand.chars().nth(card).unwrap(), joker);
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

fn hand_type(hand: &str, joker: bool) -> HandType {
    let counts = hand.chars().counts();

    if joker && hand.contains('J') {
        let joker = determine_joker(&counts);
        let hand = hand.replace('J', &joker.to_string());
        // send it back through with the replaced card
        return hand_type(&hand, false);
    }

    if hand.chars().all_equal() {
        return HandType::FiveKind;
    }
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

fn card_value(hand: char, joker: bool) -> u32 {
    hand.to_digit(10).unwrap_or_else(|| match hand {
        'T' => 10,
        'J' => joker.then_some(1).unwrap_or(11),
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unknown card value"),
    })
}

fn determine_joker(counts: &HashMap<char, usize>) -> char {
    let without_j: Vec<(&char, &usize)> = counts.iter().filter(|each| each.0 != &'J').collect();
    // two pairs need to check for the higher value
    let highest = if without_j.iter().filter(|each| each.1 == &2).count() == 2 {
        if card_value(*without_j[0].0, true) > card_value(*without_j[1].0, true) {
            without_j[0]
        } else {
            without_j[1]
        }
    } else if !without_j.is_empty() {
        *without_j.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap()
    } else {
        (&'J', &5)
    };

    *highest.0
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 5905);
    }
}

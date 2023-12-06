use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::num_traits::ToPrimitive;

type Input<'a> = Vec<Game<'a>>;

#[derive(Debug)]
pub struct Game<'a> {
    winners: HashSet<&'a str>,
    numbers: HashSet<&'a str>,
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(':').last().unwrap().split('|');
            Game {
                winners: split.next().unwrap().split_whitespace().collect(),
                numbers: split.next().unwrap().split_whitespace().collect(),
            }
        })
        .collect_vec()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|game| {
            let winners: u32 = game
                .numbers
                .intersection(&game.winners)
                .count()
                .to_u32()
                .unwrap();
            if winners > 0 {
                2_u32.pow(winners - 1)
            } else {
                0
            }
        })
        .sum()
}

// pub fn part2(input: &Input) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 13);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 467835);
    // }
}
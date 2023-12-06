use itertools::Itertools;
use pathfinding::num_traits::ToPrimitive;
use std::collections::HashSet;

type Input<'a> = Vec<Game>;

#[derive(Debug, Copy, Clone)]
pub struct Game {
    card: usize,
    winners: usize,
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let mut split = line.split(':').last().unwrap().split('|');
            let winners: HashSet<&str> = split.next().unwrap().split_whitespace().collect();
            let numbers: HashSet<&str> = split.next().unwrap().split_whitespace().collect();
            Game {
                card: index + 1,
                winners: numbers.intersection(&winners).count(),
            }
        })
        .collect_vec()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|game| {
            let winners: u32 = game.winners.to_u32().unwrap();
            if winners > 0 {
                2_u32.pow(winners - 1)
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut stack = input.clone();
    let mut size = &stack.len() - 1;
    let mut index: usize = 0;

    while index <= size {
        let game = stack[index];
        if game.winners > 0 {
            for copy in game.card..game.card + game.winners {
                stack.push(input[copy]);
            }
            size += game.winners;
        }
        index += 1;
    }

    stack.iter().len()
}

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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 30);
    }
}

use itertools::Itertools;

type Input = Vec<Vec<usize>>;

#[derive(Debug)]
pub struct Race {
    time: usize,
    distance: usize,
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn flip_arrays(lines: &Input) -> Vec<Race> {
    (0..lines[0].len())
        .map(|i| lines.iter().map(|j| j[i]).collect_vec())
        .map(|pair| Race {
            time: pair[0],
            distance: pair[1],
        })
        .collect_vec()
}

pub fn part1(input: &Input) -> usize {
    flip_arrays(input)
        .iter()
        .map(winning_combinations)
        .product()
}

pub fn part2(input: &Input) -> usize {
    let race = Race {
        time: combine_numbers(&input[0]),
        distance: combine_numbers(&input[1]),
    };
    winning_combinations(&race)
}

fn combine_numbers(numbers: &[usize]) -> usize {
    numbers
        .iter()
        .map(|x| x.to_string())
        .join("")
        .parse::<usize>()
        .unwrap()
}

fn winning_combinations(race: &Race) -> usize {
    (1..race.time - 1)
        .filter(|charge| (race.time - charge) * charge > race.distance)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 71503);
    }
}

use itertools::Itertools;

type Input = Vec<Vec<isize>>;

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> isize {
    let derivatives = input
        .iter()
        .map(|history| derivatives(history))
        .collect_vec();
    let integrals = derivatives.iter().map(integrals).collect_vec();

    integrals
        .iter()
        .map(|history| history.last().unwrap().last().unwrap())
        .sum()
}

pub fn part2(input: &Input) -> isize {
    let mut input = input.clone();
    for history in input.iter_mut() {
        history.reverse();
    }

    let derivatives = input
        .iter()
        .map(|history| derivatives(history))
        .collect_vec();
    let integrals = derivatives.iter().map(integrals).collect_vec();

    integrals
        .iter()
        .map(|history| history.last().unwrap().last().unwrap())
        .sum()
}

fn derivatives(history: &[isize]) -> Input {
    let mut current: Vec<isize> = history.to_vec();
    let mut derivatives = Vec::from([current.clone()]);

    while current.iter().sum::<isize>() != 0 {
        current = current.windows(2).map(|each| each[1] - each[0]).collect();
        derivatives.push(current.clone());
    }

    derivatives
}

fn integrals(derivatives: &Input) -> Input {
    let mut integrals = derivatives.clone();
    integrals.reverse();

    for i in 1..integrals.len() {
        let current = *integrals[i].last().unwrap();
        let previous = *integrals[i - 1].last().unwrap();
        integrals[i].push(current + previous);
    }

    integrals
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 2);
    }
}

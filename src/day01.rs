use std::str;

type Input = Vec<String>;

pub fn generator(input: &str) -> Input {
    input
        .split('\n')
        .map(|line| line.chars().filter(|char| char.is_numeric()).collect())
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|line| {
            let mut chars = line.chars();
            let mut two_chars = String::from(chars.next().unwrap());
            two_chars.push(if line.len() == 1 {
                line.parse::<char>().unwrap()
            } else {
                chars.last().unwrap()
            });
            two_chars.parse::<u32>().unwrap()
        })
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const SAMPLE_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE_2)), 281);
    }
}

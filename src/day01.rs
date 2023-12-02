use std::str;

type Input<'a> = Vec<&'a str>;

pub fn generator(input: &str) -> Input {
    input.split('\n').collect()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|line| {
            let numbers = line.chars().filter(|char| char.is_numeric()).collect();
            combine_numbers(numbers)
        })
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|line| {
            let numbers = find_numbers(line);
            combine_numbers(numbers)
        })
        .sum()
}

fn combine_numbers(numbers: Vec<char>) -> u32 {
    let mut numbers = numbers.iter();
    let mut two_chars = String::from(*numbers.next().unwrap());
    two_chars.push(if let Some(c) = numbers.last() {
        *c
    } else {
        two_chars.parse::<char>().unwrap()
    });
    two_chars.parse::<u32>().unwrap()
}

struct FoundNumber {
    index: usize,
    number: char,
}

fn find_numbers(line: &str) -> Vec<char> {
    let mut found_numbers: Vec<FoundNumber> = Vec::new();

    let mut find_number = |input: &str, number: char| {
        line.match_indices(input)
            .for_each(|(index, _)| found_numbers.push(FoundNumber { index, number }));
    };

    find_number("one", '1');
    find_number("two", '2');
    find_number("three", '3');
    find_number("four", '4');
    find_number("five", '5');
    find_number("six", '6');
    find_number("seven", '7');
    find_number("eight", '8');
    find_number("nine", '9');

    line.match_indices(|ch| ('1'..='9').contains(&ch))
        .for_each(|(index, number)| {
            found_numbers.push(FoundNumber {
                index,
                number: number.chars().next().unwrap(),
            })
        });

    found_numbers.sort_by(|a, b| a.index.cmp(&b.index));
    found_numbers.iter().map(|each| each.number).collect()
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

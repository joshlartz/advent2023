use regex::Regex;

#[derive(Debug)]
pub struct Game {
    id: u32,
    red: Vec<usize>,
    green: Vec<usize>,
    blue: Vec<usize>,
}

type Input = Vec<Game>;

pub fn generator(input: &str) -> Input {
    let cubes = Regex::new(r"(?<blue>\d+) blue|(?<red>\d+) red|(?<green>\d+) green").unwrap();
    let game = Regex::new(r"Game (\d+):").unwrap();

    input
        .lines()
        .map(|line| {
            let mut round = Game {
                id: game
                    .captures(line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
                red: Vec::new(),
                green: Vec::new(),
                blue: Vec::new(),
            };
            for caps in cubes.captures_iter(line) {
                if let Some(red) = caps.name("red") {
                    round.red.push(red.as_str().parse::<usize>().unwrap())
                }
                if let Some(green) = caps.name("green") {
                    round.green.push(green.as_str().parse::<usize>().unwrap())
                }
                if let Some(blue) = caps.name("blue") {
                    round.blue.push(blue.as_str().parse::<usize>().unwrap())
                }
            }
            round
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    const RED_MAX: usize = 12;
    const GREEN_MAX: usize = 13;
    const BLUE_MAX: usize = 14;

    input
        .iter()
        .filter(|game| {
            game.red.iter().max().unwrap_or(&0) <= &RED_MAX
                && game.green.iter().max().unwrap_or(&0) <= &GREEN_MAX
                && game.blue.iter().max().unwrap_or(&0) <= &BLUE_MAX
        })
        .fold(0, |acc, game| acc + game.id)
}

// pub fn part2(input: &Input) -> u32 {
//     input
//         .iter()
//         .map(|line| {
//             let numbers = find_numbers(line);
//             combine_numbers(numbers)
//         })
//         .sum()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 8);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE_2)), 281);
    // }
}

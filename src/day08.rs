use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

type Choices = Vec<String>;
type Network = HashMap<String, Choices>;
#[derive(Debug)]
pub struct Input {
    instructions: Vec<usize>,
    network: Network,
}

pub fn generator(input: &str) -> Input {
    let mut lines = input.lines();

    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|char| if char == 'L' { 0 } else { 1 })
        .collect_vec();

    lines.next();

    let re = Regex::new(r"[A-Z]{3}").unwrap();
    let mut network: Network = HashMap::new();
    lines.for_each(|line| {
        let mut matches = re.find_iter(line);
        network.insert(
            String::from(matches.next().unwrap().as_str()),
            matches.map(|each| String::from(each.as_str())).collect(),
        );
    });

    Input {
        instructions,
        network,
    }
}

pub fn part1(input: &Input) -> usize {
    let mut count = 0;
    let mut node = String::from("AAA");

    while node != "ZZZ" {
        follow_map(input, &mut count, &mut node);
    }

    count
}

// pub fn part2(input: &Input) -> usize {
//     main(input, true)
// }

fn follow_map(input: &Input, count: &mut usize, node: &mut String) {
    let Input {
        instructions,
        network,
    } = input;

    for step in instructions {
        *node = network.get(node).unwrap()[*step].clone();
        *count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE1)), 2);
        assert_eq!(part1(&generator(SAMPLE2)), 6);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 5905);
    // }
}

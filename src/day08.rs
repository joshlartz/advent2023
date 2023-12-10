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

    let re = Regex::new(r"[A-Z0-9]{3}").unwrap();
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

pub fn part2(input: &Input) -> usize {
    let nodes = input
        .network
        .keys()
        .filter(|each| each.ends_with('A'))
        .cloned()
        .collect_vec();

    let counts = nodes
        .iter()
        .map(|node| {
            let mut count = 0;
            let mut node = node.clone();

            while !node.ends_with('Z') {
                follow_map(input, &mut count, &mut node);
            }

            count
        })
        .collect_vec();

    lcm(&counts)
}

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

// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1A: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE1B: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE1A)), 2);
        assert_eq!(part1(&generator(SAMPLE1B)), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE2)), 6);
    }
}

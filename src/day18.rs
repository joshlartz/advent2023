use regex::Regex;
use std::fmt;

type Input = Vec<Trench>;

#[derive(Debug)]
pub struct Color(u32);
impl Color {
    pub fn from_hex_str(s: &str) -> Self {
        Self(u32::from_str_radix(s, 16).unwrap())
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:06x}", self.0)
    }
}

#[derive(Debug)]
pub struct Trench {
    direction: char,
    distance: usize,
    color: Color,
}

pub fn generator(input: &str) -> Input {
    let re = Regex::new(r"\w{6}").unwrap();

    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            Trench {
                direction: split.next().unwrap().chars().next().unwrap(),
                distance: split.next().unwrap().parse().unwrap(),
                color: Color::from_hex_str(re.find(split.next().unwrap()).unwrap().as_str()),
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    
    2
}

// pub fn part2(input: &Input) -> usize {

// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 62);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 94);
    // }
}

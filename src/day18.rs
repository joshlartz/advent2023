use itertools::Itertools;
use regex::Regex;

type Input<'a> = Vec<Trench<'a>>;

#[derive(Debug, Copy, Clone)]
struct Coord {
    row: isize,
    col: isize,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Trench<'a> {
    direction: Direction,
    distance: isize,
    color: &'a str,
    start: Coord,
    end: Coord,
}

pub fn generator(input: &str) -> Input {
    let re = Regex::new(r"\w{6}").unwrap();

    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            Trench {
                direction: match split.next().unwrap().chars().next().unwrap() {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("unknown direction"),
                },
                distance: split.next().unwrap().parse().unwrap(),
                color: re.find(split.next().unwrap()).unwrap().as_str(),
                start: Coord { row: 0, col: 0 },
                end: Coord { row: 0, col: 0 },
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> isize {
    main(input)
}

pub fn part2(input: &Input) -> isize {
    let input = input
        .iter()
        .map(|each| {
            let chunks = each
                .color
                .chars()
                .chunks(5)
                .into_iter()
                .map(|chunk| chunk.collect_vec())
                .collect_vec();
            Trench {
                direction: match chunks[1][0] {
                    '0' => Direction::Right,
                    '1' => Direction::Down,
                    '2' => Direction::Left,
                    '3' => Direction::Up,
                    _ => panic!("unknown direction"),
                },
                distance: isize::from_str_radix(&chunks[0].iter().collect::<String>(), 16).unwrap(),
                ..*each
            }
            // let distance = u32::from_str_radix(chunks[0], 16).unwrap()
        })
        .collect_vec();

    main(&input)
}

fn main(input: &Input) -> isize {
    let trenches = create_trenches(input);

    let points = trenches.iter().fold(Vec::new(), |acc: Vec<Coord>, trench| {
        [acc, [trench.start, trench.end].to_vec()].concat()
    });

    let area = points.windows(2).fold(0, |acc, points| {
        acc + points[0].col * points[1].row - points[0].row * points[1].col
    }) / 2;

    let border = trenches.iter().fold(0, |acc, trench| acc + trench.distance);

    area + border / 2 + 1
}

fn create_trenches<'a>(input: &'a Input<'a>) -> Input<'a> {
    let mut row: isize = 0;
    let mut col: isize = 0;
    input
        .iter()
        .map(|trench| {
            let start = Coord { row, col };
            match trench.direction {
                Direction::Up => row -= trench.distance,
                Direction::Down => row += trench.distance,
                Direction::Left => col -= trench.distance,
                Direction::Right => col += trench.distance,
            }
            Trench {
                start,
                end: Coord { row, col },
                ..*trench
            }
        })
        .collect()
}

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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 952_408_144_115);
    }
}

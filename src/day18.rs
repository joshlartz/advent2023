use itertools::Itertools;
use pathfinding::{num_traits::ToPrimitive, prelude::Matrix};
use regex::Regex;
use std::{collections::HashMap, fmt};

type Input = Vec<Trench>;

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

#[derive(Debug, Copy, Clone)]
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
    direction: Direction,
    distance: isize,
    color: Color,
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
                color: Color::from_hex_str(re.find(split.next().unwrap()).unwrap().as_str()),
                start: Coord { row: 0, col: 0 },
                end: Coord { row: 0, col: 0 },
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> isize {
    let trenches = create_trenches(input);

    let points = trenches.iter().fold(Vec::new(), |acc: Vec<Coord>, trench| {
        [acc, [trench.start, trench.end].to_vec()].concat()
    });

    let area = points.windows(2).fold(0, |acc, points| {
        acc + points[0].col * points[1].row - points[0].row * points[1].col
    }) / 2;

    let border = trenches.iter().fold(0, |acc, trench| { acc + trench.distance });

    area + border / 2 + 1
}

// pub fn part2(input: &Input) -> usize {

// }

fn create_trenches(input: &Input) -> Input {
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

#[derive(Debug)]
struct MinMax {
    row_min: isize,
    row_max: isize,
    col_min: isize,
    col_max: isize,
}

fn find_grid_size(input: &Input) -> MinMax {
    let min_max = input
        .iter()
        .map(|trench| MinMax {
            row_min: trench.start.row.min(trench.end.row),
            row_max: trench.start.row.max(trench.end.row),
            col_min: trench.start.col.min(trench.end.col),
            col_max: trench.start.col.max(trench.end.col),
        })
        .collect_vec();
    MinMax {
        row_min: min_max
            .iter()
            .min_by(|a, b| a.row_min.cmp(&b.row_min))
            .unwrap()
            .row_min,
        row_max: min_max
            .iter()
            .max_by(|a, b| a.row_max.cmp(&b.row_max))
            .unwrap()
            .row_max,
        col_min: min_max
            .iter()
            .min_by(|a, b| a.col_min.cmp(&b.col_min))
            .unwrap()
            .col_min,
        col_max: min_max
            .iter()
            .max_by(|a, b| a.col_max.cmp(&b.col_max))
            .unwrap()
            .col_max,
    }
}

fn dig_trenches(trenches: Vec<Trench>) -> Matrix<Option<Color>> {
    let grid_size = find_grid_size(&trenches);
    let row_offset = 0 - grid_size.row_min;
    let col_offset = 0 - grid_size.col_min;

    let mut matrix: Matrix<Option<Color>> = Matrix::new(
        (grid_size.row_max + row_offset + 1).to_usize().unwrap(),
        (grid_size.col_max + col_offset + 1).to_usize().unwrap(),
        None,
    );

    trenches.iter().for_each(|trench| match trench.direction {
        Direction::Down => (trench.start.row..=trench.end.row).for_each(|row| {
            matrix[(
                (row + row_offset).to_usize().unwrap(),
                (trench.end.col + col_offset).to_usize().unwrap(),
            )] = Some(trench.color)
        }),
        Direction::Up => (trench.end.row..=trench.start.row).for_each(|row| {
            matrix[(
                (row + row_offset).to_usize().unwrap(),
                (trench.end.col + col_offset).to_usize().unwrap(),
            )] = Some(trench.color)
        }),
        Direction::Left => (trench.end.col..=trench.start.col).for_each(|col| {
            matrix[(
                (trench.end.row + row_offset).to_usize().unwrap(),
                (col + col_offset).to_usize().unwrap(),
            )] = Some(trench.color)
        }),
        Direction::Right => (trench.start.col..=trench.end.col).for_each(|col| {
            matrix[(
                (trench.end.row + row_offset).to_usize().unwrap(),
                (col + col_offset).to_usize().unwrap(),
            )] = Some(trench.color)
        }),
    });

    matrix
}

fn dig_out_interior(matrix: &mut Matrix<Option<Color>>) {
    let mut interior: HashMap<(usize, usize), Option<Color>> = HashMap::new();

    for (row, colors) in matrix.iter().enumerate() {
        let mut inside = false;
        let mut last_seen: Option<Color> = None;
        let mut count = 0;

        for (col, color) in colors.iter().enumerate() {
            if color.is_some() {
                count += 1;
            }
            if color.is_none() && last_seen.is_some() && count == 1 {
                inside ^= true;
                count = 0;
            }
            if color.is_none() && inside {
                interior.insert((row, col), Some(Color::from_hex_str("000000")));
            }
            last_seen = *color;
        }
    }

    interior.iter().for_each(|(k, v)| matrix[*k] = *v);
}

#[allow(dead_code)]
fn print_grid(matrix: &Matrix<Option<Color>>) {
    for row in matrix.iter() {
        println!(
            "{:?}",
            row.iter()
                .map(|t| if t.is_some() { '#' } else { '.' })
                .join("")
        );
    }
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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 94);
    // }
}

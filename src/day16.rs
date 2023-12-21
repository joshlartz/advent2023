use std::collections::HashSet;

use grid::Grid;
use itertools::Itertools;

type Input = Grid<char>;

#[derive(Debug)]
struct Beam {
    direction: Direction,
    position: Coord,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

pub fn generator(input: &str) -> Input {
    let mut grid = Grid::new(0, 0);

    for line in input.lines() {
        grid.push_row(line.chars().collect_vec());
    }

    grid
}

pub fn part1(input: &Input) -> usize {
    main(0, 0, Direction::Right, input)
}

pub fn part2(input: &Input) -> usize {
    let cols = 0..input.cols();
    let rows = 0..input.rows();

    let left = rows
        .clone()
        .map(|row| main(row, input.cols() - 1, Direction::Left, input))
        .max()
        .unwrap();
    let right = rows
        .map(|row| main(row, 0, Direction::Right, input))
        .max()
        .unwrap();
    let up = cols
        .clone()
        .map(|col| main(input.rows() - 1, col, Direction::Up, input))
        .max()
        .unwrap();
    let down = cols
        .map(|col| main(0, col, Direction::Down, input))
        .max()
        .unwrap();

    *[left, right, up, down].iter().max().unwrap()
}

fn main(row: usize, col: usize, direction: Direction, input: &Input) -> usize {
    let mut engergized_grid = Grid::init(input.rows(), input.cols(), HashSet::with_capacity(4));

    let mut start = Beam {
        position: Coord { row, col },
        direction,
    };
    start.direction = bend_light(&start, &input[(row, col)])[0];

    move_beam(input, &mut engergized_grid, start);

    engergized_grid
        .iter()
        .fold(0, |acc, tile| acc + if !tile.is_empty() { 1 } else { 0 })
}

fn move_beam(input: &Input, grid: &mut Grid<HashSet<Direction>>, beam: Beam) {
    if grid[(beam.position.row, beam.position.col)].contains(&beam.direction) {
        return;
    }
    grid[(beam.position.row, beam.position.col)].insert(beam.direction);

    if let Some(next) = next_tile(&beam, input) {
        let directions = bend_light(&beam, &input[(next.row, next.col)]);

        directions.iter().for_each(|direction| {
            move_beam(
                input,
                grid,
                Beam {
                    direction: *direction,
                    position: next,
                },
            )
        });
    }
}

fn next_tile(beam: &Beam, input: &Input) -> Option<Coord> {
    match beam.direction {
        Direction::Left => {
            if beam.position.col > 0 {
                Some(Coord {
                    col: beam.position.col - 1,
                    row: beam.position.row,
                })
            } else {
                None
            }
        }
        Direction::Right => {
            if beam.position.col < input.cols() - 1 {
                Some(Coord {
                    col: beam.position.col + 1,
                    row: beam.position.row,
                })
            } else {
                None
            }
        }
        Direction::Up => {
            if beam.position.row > 0 {
                Some(Coord {
                    col: beam.position.col,
                    row: beam.position.row - 1,
                })
            } else {
                None
            }
        }
        Direction::Down => {
            if beam.position.row < input.rows() - 1 {
                Some(Coord {
                    col: beam.position.col,
                    row: beam.position.row + 1,
                })
            } else {
                None
            }
        }
    }
}

fn bend_light(beam: &Beam, tile: &char) -> Vec<Direction> {
    match tile {
        '.' => vec![beam.direction],
        '/' => match beam.direction {
            Direction::Left => vec![Direction::Down],
            Direction::Right => vec![Direction::Up],
            Direction::Up => vec![Direction::Right],
            Direction::Down => vec![Direction::Left],
        },
        '\\' => match beam.direction {
            Direction::Left => vec![Direction::Up],
            Direction::Right => vec![Direction::Down],
            Direction::Up => vec![Direction::Left],
            Direction::Down => vec![Direction::Right],
        },
        '-' => match beam.direction {
            Direction::Left | Direction::Right => vec![beam.direction],
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
        },
        '|' => match beam.direction {
            Direction::Up | Direction::Down => vec![beam.direction],
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
        },
        _ => panic!("unknown tile type: {} at {:?}", tile, beam.position),
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid<HashSet<Direction>>) {
    for row in 0..grid.rows() {
        println!(
            "{:?}",
            grid.iter_row(row)
                .map(|tile| if tile.is_empty() { '.' } else { '#' })
                .join("")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 51);
    }
}

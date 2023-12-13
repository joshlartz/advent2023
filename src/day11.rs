use grid::*;
use itertools::Itertools;

pub struct Input {
    galaxies: Vec<Galaxy>,
    grid: Grid<char>,
}

#[derive(Debug, Clone)]
struct Coord {
    row: usize,
    col: usize,
}
// because better name
type Galaxy = Coord;

pub fn generator(input: &str) -> Input {
    let mut grid = Grid::new(0, 0);

    for line in input.lines() {
        grid.push_row(line.chars().collect_vec());
    }

    let galaxies = grid
        .iter()
        .enumerate()
        .filter_map(|(i, each)| {
            if each == &'#' {
                Some(Galaxy {
                    row: i / grid.rows(),
                    col: i % grid.rows(),
                })
            } else {
                None
            }
        })
        .collect_vec();

    Input { galaxies, grid }
}

pub fn part1(input: &Input) -> isize {
    main(input, 1)
}

pub fn part2(input: &Input) -> isize {
    main(input, 999_999)
}

fn main(input: &Input, by: usize) -> isize {
    let mut galaxies = input.galaxies.clone();

    expand_the_universe(&mut galaxies, by, &input.grid);

    let combinations = (0..galaxies.len()).tuple_combinations();

    combinations
        .map(|(a, b)| manhattan_distance(&galaxies[a], &galaxies[b]))
        .sum()
}

fn expand_the_universe(galaxies: &mut Vec<Coord>, by: usize, grid: &Grid<char>) {
    let mut empty_rows = grid.iter_rows().enumerate().filter_map(|(i, mut row)| {
        if row.all(|x| x == &'.') {
            Some(i)
        } else {
            None
        }
    }).collect_vec();
    let mut empty_cols = grid.iter_cols().enumerate().filter_map(|(i, mut col)| {
        if col.all(|x| x == &'.') {
            Some(i)
        } else {
            None
        }
    }).collect_vec();

    empty_rows.reverse();
    empty_cols.reverse();

    empty_rows.iter().for_each(|row| {
        galaxies.iter_mut().for_each(|galaxy| {
            if galaxy.row > *row {
                galaxy.row += by
            }
        })
    });
    empty_cols.iter().for_each(|col| {
        galaxies.iter_mut().for_each(|galaxy| {
            if galaxy.col > *col {
                galaxy.col += by
            }
        })
    });
}

fn manhattan_distance(a: &Coord, b: &Coord) -> isize {
    (a.col.abs_diff(b.col) + a.row.abs_diff(b.row))
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(main(&generator(SAMPLE), 9), 1030);
        assert_eq!(main(&generator(SAMPLE), 99), 8410);
    }
}

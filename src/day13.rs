use itertools::Itertools;
use pathfinding::num_traits::ToPrimitive;

type Input = Vec<Mirrors>;

#[derive(Debug)]
pub struct Mirrors {
    rows: Vec<String>,
    cols: Vec<String>,
}

pub fn generator(input: &str) -> Input {
    let mut input = input
        .split("\n\n")
        .map(|each| Mirrors {
            rows: each.lines().map(|x| x.into()).collect(),
            cols: vec![],
        })
        .collect_vec();

    input
        .iter_mut()
        .for_each(|mirrors| mirrors.cols = make_cols(&mirrors.rows));

    input
}

/** swap rows for columns so columns can also be sliced for searching */
fn make_cols(rows: &[String]) -> Vec<String> {
    let col_max = rows[0].len();
    let mut cols: Vec<String> = vec![String::new(); col_max];

    for row in rows.iter() {
        for i in 0..col_max {
            cols[i].push_str(&row[i..=i]);
        }
    }

    cols
}

pub fn part1(input: &Input) -> isize {
    input
        .iter()
        .map(|mirrors| {
            match_row(&mirrors.rows, false)
                .unwrap_or_else(|| match_column(&mirrors.cols, false).unwrap())
        })
        .sum()
}

pub fn part2(input: &Input) -> isize {
    input
        .iter()
        .map(|mirrors| {
            match_row(&mirrors.rows, true)
                .unwrap_or_else(|| match_column(&mirrors.cols, true).unwrap())
        })
        .sum()
}

fn find_reflection(mirrors: &[String], multiplier: isize, smudge: bool) -> Option<isize> {
    for (i, _windows) in mirrors.windows(2).enumerate() {
        let i: isize = i.try_into().unwrap();
        if check_outward(mirrors, (i, i + 1)) == if smudge { 1 } else { 0 } {
            return Some((i + 1) * multiplier);
        }
    }

    None
}

fn match_column(mirrors: &[String], smudge: bool) -> Option<isize> {
    find_reflection(mirrors, 1, smudge)
}

fn match_row(mirrors: &[String], smudge: bool) -> Option<isize> {
    find_reflection(mirrors, 100, smudge)
}

/** true if they are mirrored */
fn check_outward(mirrors: &[String], indexes: (isize, isize)) -> usize {
    if indexes.0 < 0 || indexes.1 >= mirrors.len().to_isize().unwrap() {
        return 0;
    }

    let mismatches = mirrors[indexes.0.to_usize().unwrap()]
        .chars()
        .zip(mirrors[indexes.1.to_usize().unwrap()].chars())
        .filter(|(a, b)| a != b)
        .count();
    mismatches + check_outward(mirrors, (indexes.0 - 1, indexes.1 + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 400);
    }
}

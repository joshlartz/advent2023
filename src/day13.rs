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
            match_row(&mirrors.rows).unwrap_or_else(|| match_column(&mirrors.cols).unwrap())
        })
        .sum()
}

// pub fn part2(input: &Input) -> usize {
//
// }

fn find_reflection(mirrors: &[String], multiplier: isize) -> Option<isize> {
    let mut reflection_points: Vec<(isize, isize)> = Vec::new();
    for (i, windows) in mirrors.windows(2).enumerate() {
        if windows[0] == windows[1] {
            let i = i.try_into().unwrap();
            reflection_points.push((i, i + 1));
        }
    }
    // println!("{} reflection_points: {:?}", if multiplier == 100 { "horizontal" } else { "vertical" }, reflection_points);
    if !reflection_points.is_empty() {
        for reflection_point in reflection_points.iter() {
            if check_outward(mirrors, (reflection_point.0 - 1, reflection_point.1 + 1)) {
                return Some(reflection_point.1 * multiplier);
            }
        }
    }

    None
}

fn match_column(mirrors: &[String]) -> Option<isize> {
    find_reflection(mirrors, 1)
}

fn match_row(mirrors: &[String]) -> Option<isize> {
    find_reflection(mirrors, 100)
}

/** true if they are mirrored */
fn check_outward(mirrors: &[String], indexes: (isize, isize)) -> bool {
    if indexes.0 < 0 || indexes.1 >= mirrors.len().to_isize().unwrap() {
        return true;
    }

    if mirrors[indexes.0.to_usize().unwrap()] == mirrors[indexes.1.to_usize().unwrap()] {
        return check_outward(mirrors, (indexes.0 - 1, indexes.1 + 1));
    }

    false
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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 525152);
    // }
}

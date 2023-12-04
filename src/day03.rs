use itertools::Itertools;
use regex::Regex;
use std::ops::RangeInclusive;

type Input<'a> = (Vec<&'a str>, Vec<Part>);

#[derive(Debug)]
pub struct Part {
    number: usize,
    start: usize,
    end: usize,
    row: usize,
}

#[derive(Debug)]
struct Adjacency {
    row: usize,
    range: RangeInclusive<usize>,
}

impl Part {
    fn adjacencies(&self) -> Vec<Adjacency> {
        let left = if self.start == 0 { 0 } else { self.start - 1 };

        // right side
        let mut adjacencies = vec![Adjacency {
            row: self.row,
            range: self.end + 1..=self.end + 1,
        }];
        // left side
        if left < self.start {
            adjacencies.push(Adjacency {
                row: self.row,
                range: left..=left,
            });
        }
        // above row
        if self.row > 0 {
            adjacencies.push(Adjacency {
                row: self.row - 1,
                range: left..=self.end + 1,
            })
        };
        // below row
        adjacencies.push(Adjacency {
            row: self.row + 1,
            range: left..=self.end + 1,
        });

        adjacencies
    }
}

pub fn generator(input: &str) -> Input {
    let re = Regex::new(r"\d+").unwrap();

    (
        input.lines().collect_vec(),
        input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                re.find_iter(line).map(move |m| Part {
                    number: m.as_str().parse().unwrap(),
                    start: m.start(),
                    end: m.end() - 1, // used for inclusive ranges
                    row,
                })
            })
            .collect(),
    )
}

pub fn part1(input: &Input) -> usize {
    let (rows, parts) = input;

    let col_max = rows[0].len() - 1;

    let symbols = Regex::new(r"[^\.\d]").unwrap();

    parts
        .iter()
        .filter_map(|part| {
            if part.adjacencies().iter().any(|adj| {
                // check for row out of bounds
                if let Some(row) = rows.get(adj.row) {
                    // check for column out of bounds
                    if adj.range.start() > &col_max { return false };
                    let range = if adj.range.end() > &col_max {
                        *adj.range.start()..=(adj.range.end() - 1)
                    } else {
                        adj.range.clone()
                    };
                    symbols.is_match(&row[range])
                } else {
                    false
                }
            }) {
                Some(part.number)
            } else {
                None
            }
        })
        .sum()
}

// pub fn part2(input: &Input) -> usize {

// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 4361);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 2286);
    // }
}

use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, ops::RangeInclusive};

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
                    if adj.range.start() > &col_max {
                        return false;
                    };
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

#[derive(Debug)]
struct Gear {
    row: usize,
    col: usize,
}

impl Gear {
    fn adjacencies(&self) -> Vec<Adjacency> {
        // right side
        let mut adjacencies = vec![Adjacency {
            row: self.row,
            range: self.col + 1..=self.col + 1,
        }];
        // left side
        adjacencies.push(Adjacency {
            row: self.row,
            range: self.col - 1..=self.col - 1,
        });

        // above row
        adjacencies.push(Adjacency {
            row: self.row - 1,
            range: self.col - 1..=self.col + 1,
        });

        // below row
        adjacencies.push(Adjacency {
            row: self.row + 1,
            range: self.col - 1..=self.col + 1,
        });

        adjacencies
    }
}

pub fn part2(input: &Input) -> usize {
    let (rows, parts) = input;

    let grouped_parts = parts.iter().fold(HashMap::new(), |mut map, part| {
        map.entry(part.row).or_insert_with(|| Vec::new()).push(part);
        map
    });

    let re = Regex::new(r"\*").unwrap();
    let gears = rows
        .iter()
        .enumerate()
        .flat_map(|(index, row)| {
            re.find_iter(row).map(move |m| Gear {
                row: index,
                col: m.start(),
            })
        })
        .collect_vec();
    // println!("{:?}", gears[0].adjacencies());

    gears
        .iter()
        .map(|gear| {
            gear.adjacencies()
                .iter()
                .filter_map(|adj| {
                    if let Some(row) = &grouped_parts.get(&adj.row) {
                        Some(
                            row.iter()
                                .filter(|part| {
                                    (adj.range.start() <= &part.end)
                                        && &part.start <= adj.range.end()
                                })
                                .collect_vec(),
                        )
                    } else {
                        None
                    }
                })
                .flatten()
                .collect_vec()
        })
        .filter_map(|part| {
            if part.len() == 2 {
                Some(part[0].number * part[1].number)
            } else {
                None
            }
        })
        .sum()
}

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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 467835);
    }
}

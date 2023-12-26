use itertools::Itertools;
use pathfinding::{
    num_traits::ToPrimitive,
    prelude::{astar, Matrix},
};
use std::hash::{Hash, Hasher};

type Input = Matrix<u32>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord {
    row: usize,
    col: usize,
}
impl Coord {
    fn direction(&self, destination: (usize, usize)) -> Direction {
        let row_diff: isize = self.row.to_isize().unwrap() - destination.0.to_isize().unwrap();
        let col_diff: isize = self.col.to_isize().unwrap() - destination.1.to_isize().unwrap();

        if row_diff > 0 {
            return Direction::Up;
        }
        if row_diff < 0 {
            return Direction::Down;
        }
        if col_diff > 0 {
            return Direction::Left;
        }
        if col_diff < 0 {
            return Direction::Right;
        }
        panic!(
            "invalid direction - source: {:?}, destination: {:?}",
            self, destination
        );
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Eq)]
struct Block {
    coord: Coord,
    blocks: usize,
    direction: Direction,
}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord
            && self.direction == other.direction
            && self.blocks == other.blocks
    }
}
impl Hash for Block {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.coord.hash(hasher);
        self.direction.hash(hasher);
        self.blocks.hash(hasher);
    }
}

struct Map {
    grid: Matrix<u32>,
    start: Block,
    end: Coord,
}
impl Map {
    fn new(input: &Input) -> Self {
        let grid = input.clone();
        let start = Block {
            coord: Coord { row: 0, col: 0 },
            blocks: 0,
            direction: Direction::Down,
        };
        let end = Coord {
            row: input.rows - 1,
            col: input.columns - 1,
        };

        Self { grid, start, end }
    }

    fn eligible_neighbors(&self, block: &Block) -> Vec<(Block, u32)> {
        self
            .grid
            .neighbours((block.coord.row, block.coord.col), false)
            .map(|neighbor| {
                let direction = block.coord.direction(neighbor);
                let blocks = if block.direction == direction {
                    block.blocks + 1
                } else {
                    1
                };

                (
                    Block {
                        coord: Coord {
                            row: neighbor.0,
                            col: neighbor.1,
                        },
                        blocks,
                        direction,
                    },
                    self.grid[neighbor], // heat loss
                )
            })
            .filter(|neighbor| {
                neighbor.0.blocks <= 3
                    && match block.direction {
                        Direction::Down => neighbor.0.direction != Direction::Up,
                        Direction::Up => neighbor.0.direction != Direction::Down,
                        Direction::Left => neighbor.0.direction != Direction::Right,
                        Direction::Right => neighbor.0.direction != Direction::Left,
                    }
            })
            .collect()
    }

    fn heuristic(&self, block: &Block) -> u32 {
        let a = self.end.row.abs_diff(block.coord.row);
        let b = self.end.col.abs_diff(block.coord.col);
        // an actual use for the pythagorean theorom lol
        f32::sqrt((a.pow(2) + b.pow(2)) as f32).to_u32().unwrap()
    }
}

pub fn generator(input: &str) -> Input {
    Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .unwrap()
}

pub fn part1(input: &Input) -> u32 {
    let map = Map::new(input);

    let path = astar(
        &map.start,
        |block| map.eligible_neighbors(block),
        |block| map.heuristic(block),
        |block| map.end.eq(&block.coord),
    )
    .unwrap_or_else(|| panic!("no path found"));

    // print_grid(&map.grid, &path.0);
    path.1
}

// pub fn part2(input: &Input) -> usize {

// }

#[allow(dead_code)]
fn print_grid(matrix: &Input, path: &[Block]) {
    let mut grid = matrix
        .iter()
        .map(|row: &[u32]| row.iter().map(|c| c.to_string()).collect_vec())
        .collect_vec();

    path.iter().for_each(|block| {
        grid[block.coord.row][block.coord.col] = match block.direction {
            Direction::Up => String::from("^"),
            Direction::Down => String::from("v"),
            Direction::Left => String::from("<"),
            Direction::Right => String::from(">"),
        }
    });

    for row in grid.iter() {
        println!("{:?}", row.iter().join(""));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 102);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 51);
    // }
}

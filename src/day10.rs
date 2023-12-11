use grid::*;
use itertools::Itertools;

pub struct Input {
    pipes: Pipes,
    start: Coord,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    #[default]
    Ground,
    Start,
    Visited,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    row: usize,
    col: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Location {
    coord: Coord,
    pipe: Pipe,
}

#[derive(Debug, Clone)]
struct Pipes {
    grid: Grid<Pipe>,
}

impl Pipes {
    fn check_neighbor(&self, coord: &Coord, direction: &Direction) -> Option<Coord> {
        let mut neighbor: Option<Coord> = None;

        match direction {
            Direction::Up => {
                if coord.row > 0 {
                    if let Some(pipe) = self.grid.get(coord.row - 1, coord.col) {
                        if [Pipe::Vertical, Pipe::SouthEast, Pipe::SouthWest].contains(pipe) {
                            neighbor = Some(Coord {
                                row: coord.row - 1,
                                col: coord.col,
                            })
                        }
                    }
                }
            }
            Direction::Down => {
                if coord.row < self.grid.rows() - 1 {
                    if let Some(pipe) = self.grid.get(coord.row + 1, coord.col) {
                        if [Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest].contains(pipe) {
                            neighbor = Some(Coord {
                                row: coord.row + 1,
                                col: coord.col,
                            })
                        }
                    }
                }
            }
            Direction::Left => {
                if coord.col > 0 {
                    if let Some(pipe) = self.grid.get(coord.row, coord.col - 1) {
                        if [Pipe::Horizontal, Pipe::SouthEast, Pipe::NorthEast].contains(pipe) {
                            neighbor = Some(Coord {
                                row: coord.row,
                                col: coord.col - 1,
                            })
                        }
                    }
                }
            }
            Direction::Right => {
                if coord.col < self.grid.cols() - 1 {
                    if let Some(pipe) = self.grid.get(coord.row, coord.col + 1) {
                        if [Pipe::Horizontal, Pipe::SouthWest, Pipe::NorthWest].contains(pipe) {
                            neighbor = Some(Coord {
                                row: coord.row,
                                col: coord.col + 1,
                            })
                        }
                    }
                }
            }
        }

        neighbor
    }

    fn find_neighbors(&self, location: &Location) -> Vec<Location> {
        let neighbors = self
            .eligible_neighbors(&location.pipe)
            .iter()
            .filter_map(|direction| self.check_neighbor(&location.coord, direction))
            .map(|coord| Location {
                coord,
                pipe: self.get_pipe(&coord),
            })
            .collect_vec();
        if neighbors.is_empty() {
            // print_grid(&self.grid);
            panic!("{:?} had no neighbors", location.coord);
        };

        neighbors
    }

    fn visisted(&mut self, coord: &Coord) {
        self.grid[(coord.row, coord.col)] = Pipe::Visited
    }

    fn get_pipe(&self, coord: &Coord) -> Pipe {
        self.grid[(coord.row, coord.col)]
    }

    /// valid directions to move in
    fn eligible_neighbors(&self, pipe: &Pipe) -> Vec<Direction> {
        match pipe {
            Pipe::Vertical => vec![Direction::Up, Direction::Down],
            Pipe::Horizontal => vec![Direction::Left, Direction::Right],
            Pipe::NorthEast => vec![Direction::Up, Direction::Right],
            Pipe::NorthWest => vec![Direction::Up, Direction::Left],
            Pipe::SouthWest => vec![Direction::Down, Direction::Left],
            Pipe::SouthEast => vec![Direction::Down, Direction::Right],
            Pipe::Start => vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
            _ => panic!("Unknown pipe type"),
        }
    }
}

pub fn generator(input: &str) -> Input {
    let mut grid = Grid::new(0, 0);

    for line in input.lines() {
        grid.push_row(line.chars().map(pipe_type).collect_vec());
    }

    Input {
        start: find_start(&grid),
        pipes: Pipes { grid },
    }
}

fn pipe_type(char: char) -> Pipe {
    match char {
        '|' => Pipe::Vertical,
        '-' => Pipe::Horizontal,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        '.' => Pipe::Ground,
        'S' => Pipe::Start,
        _ => panic!("Unknown pipe type"),
    }
}

pub fn part1(input: &Input) -> usize {
    let mut pipes = input.pipes.clone();

    let mut count: usize = 1;
    let mut paths = pipes.find_neighbors(&Location {
        coord: input.start,
        pipe: Pipe::Start,
    });

    paths
        .iter()
        .for_each(|location| pipes.visisted(&location.coord));

    while paths[0].coord != paths[1].coord {
        let next = paths
            .iter()
            .map(|location| pipes.find_neighbors(location)[0])
            .collect_vec();
        paths.iter_mut().enumerate().for_each(|(i, location)| {
            pipes.visisted(&next[i].coord);
            *location = next[i];
        });
        count += 1;
    }

    count
}

// pub fn part2(input: &Input) -> usize {
//
// }

fn find_start(input: &Grid<Pipe>) -> Coord {
    let mut start: Option<Coord> = None;
    for row in 0..input.size().0 {
        if let Some(col) = input
            .iter_row(row)
            .find_position(|pipe| *pipe == &Pipe::Start)
        {
            start = Some(Coord { row, col: col.0 });
            break;
        }
    }
    start.unwrap_or_else(|| panic!("no start was found"))
}

#[allow(dead_code)]
fn reverse_pipe(pipe: &Pipe) -> char {
    match pipe {
        Pipe::Vertical => '|',
        Pipe::Horizontal => '-',
        Pipe::NorthEast => 'L',
        Pipe::NorthWest => 'J',
        Pipe::SouthWest => '7',
        Pipe::SouthEast => 'F',
        Pipe::Ground => '.',
        Pipe::Start => 'S',
        Pipe::Visited => 'x'
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid<Pipe>) {
    for row in 0..grid.rows() {
        println!("{:?}", grid.iter_row(row).map(reverse_pipe).join(""));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const SAMPLE2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE1)), 4);
        assert_eq!(part1(&generator(SAMPLE2)), 8);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 2);
    // }
}

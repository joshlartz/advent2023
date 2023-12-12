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
    let mut pipes: Pipes = input.pipes.clone();

    visit_loop(&mut pipes, &input.start)
}

pub fn part2(input: &Input) -> usize {
    let mut input_grid = input.pipes.grid.to_owned();
    let mut visited_pipes: Pipes = input.pipes.clone();

    let start_pipe = get_start_pipe(&visited_pipes, &input.start);

    visit_loop(&mut visited_pipes, &input.start);

    // swap start for a real pipe
    input_grid[(input.start.row, input.start.col)] = start_pipe;

    let mut count = 0;

    for row in 0..input_grid.rows() {
        let mut inside = false;

        count += input_grid
            .iter_row(row)
            .enumerate()
            .filter(|(col, pipe)| {
                if should_flip(pipe) && visited_pipes.grid.get(row, *col).unwrap() == &Pipe::Visited
                {
                    inside ^= true
                }
                inside && visited_pipes.grid.get(row, *col).unwrap() != &Pipe::Visited
            })
            .count();
    }

    count
}

fn visit_loop(pipes: &mut Pipes, start: &Coord) -> usize {
    let mut count: usize = 1;
    let mut paths = pipes.find_neighbors(&Location {
        coord: *start,
        pipe: Pipe::Start,
    });

    pipes.visisted(start);
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

fn get_start_pipe(pipes: &Pipes, start: &Coord) -> Pipe {
    let binding = pipes.eligible_neighbors(&Pipe::Start);
    let directions = binding
        .iter()
        .filter(|direction| pipes.check_neighbor(start, direction).is_some())
        .collect_vec();

    match directions[..] {
        [Direction::Up, Direction::Down] => Pipe::Vertical,
        [Direction::Left, Direction::Right] => Pipe::Horizontal,
        [Direction::Up, Direction::Right] => Pipe::NorthEast,
        [Direction::Up, Direction::Left] => Pipe::NorthWest,
        [Direction::Down, Direction::Left] => Pipe::SouthWest,
        [Direction::Down, Direction::Right] => Pipe::SouthEast,
        _ => panic!("unknown direction combination"),
    }
}

fn should_flip(pipe: &Pipe) -> bool {
    [Pipe::Vertical, Pipe::NorthWest, Pipe::NorthEast].contains(pipe)
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
        Pipe::Visited => 'x',
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

    const SAMPLE1A: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const SAMPLE1B: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const SAMPLE2A: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const SAMPLE2B: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const SAMPLE2C: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE1A)), 4);
        assert_eq!(part1(&generator(SAMPLE1B)), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE2A)), 4);
        assert_eq!(part2(&generator(SAMPLE2B)), 8);
        assert_eq!(part2(&generator(SAMPLE2C)), 10);
    }
}

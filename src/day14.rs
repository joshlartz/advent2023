use itertools::Itertools;
use std::collections::HashMap;

type Input = Vec<Vec<char>>;

pub fn generator(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(input: &Input) -> usize {
    let mut grid = input.clone();
    tilt_north(&mut grid);

    support_beams(&grid)
}

pub fn part2(input: &Input) -> usize {
    const MAX: usize = 1_000_000_000;
    let mut grid = input.clone();

    let mut unique_cycles: HashMap<String, usize> = HashMap::new();
    let mut cycle_size: usize = 0;
    let mut cycle: usize = 0;

    for i in 0..1_000_000_000 {
        spin_cycle(&mut grid);

        let key = stringify(&grid);
        if let Some(value) = unique_cycles.get(&key) {
            cycle = i;
            cycle_size = i - value;
            break;
        } else {
            unique_cycles.insert(key.clone(), i);
        }
    }
    // println!("cycle start: {}", cycle_start);
    // println!("cycle size: {}", cycle_size);
    // println!("cycles remaining: {}", (MAX - cycle) % cycle_size);

    for _ in 0..(MAX - cycle - 1) % cycle_size {
        spin_cycle(&mut grid);
    }

    support_beams(&grid)
}

fn spin_cycle(grid: &mut Input) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn tilt_north(grid: &mut Input) {
    fn move_up(grid: &mut Input, row: usize, col: usize) {
        if grid[row][col] == 'O' && row > 0 && grid[row - 1][col] == '.' {
            grid[row - 1][col] = 'O';
            grid[row][col] = '.';
            move_up(grid, row - 1, col);
        }
    }

    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            move_up(grid, row, col);
        }
    }
}

fn tilt_west(grid: &mut Input) {
    fn move_left(grid: &mut Input, row: usize, col: usize) {
        if grid[row][col] == 'O' && col > 0 && grid[row][col - 1] == '.' {
            grid[row][col - 1] = 'O';
            grid[row][col] = '.';
            move_left(grid, row, col - 1);
        }
    }

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            move_left(grid, row, col);
        }
    }
}

fn tilt_south(grid: &mut Input) {
    fn move_down(grid: &mut Input, row: usize, col: usize) {
        if grid[row][col] == 'O' && row < grid.len() - 1 && grid[row + 1][col] == '.' {
            grid[row + 1][col] = 'O';
            grid[row][col] = '.';
            move_down(grid, row + 1, col);
        }
    }

    for col in 0..grid[0].len() {
        for row in (0..grid.len()).rev() {
            move_down(grid, row, col);
        }
    }
}

fn tilt_east(grid: &mut Input) {
    fn move_right(grid: &mut Input, row: usize, col: usize) {
        if grid[row][col] == 'O' && col < grid[row].len() - 1 && grid[row][col + 1] == '.' {
            grid[row][col + 1] = 'O';
            grid[row][col] = '.';
            move_right(grid, row, col + 1);
        }
    }

    for row in 0..grid.len() {
        for col in (0..grid[0].len()).rev() {
            move_right(grid, row, col);
        }
    }
}

fn support_beams(grid: &Input) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|char| *char == &'O').count() * (i + 1))
        .sum()
}

#[allow(dead_code)]
fn print_grid(grid: &Input) {
    for row in grid.iter() {
        println!("{:?}", row.iter().join(""));
    }
    println!(" ");
}

fn stringify(grid: &Input) -> String {
    grid.iter()
        .fold(String::from(""), |mut acc: String, row: &Vec<char>| {
            acc.push_str(&row.iter().join(""));
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 64);
    }
}

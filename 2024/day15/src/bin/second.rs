#![feature(test)]

use std::{collections::VecDeque, fs, ops::Add};

use itertools::Itertools;
use ndarray::Array2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    CrateLeft,
    CrateRight,
    Immovable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Coordinates((usize, usize));

impl Add<Direction> for Coordinates {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            // Because we have walls around the grid, we don't need to check for
            // out-of-bounds
            Direction::Up => Coordinates((self.0 .0 - 1, self.0 .1)),
            Direction::Down => Coordinates((self.0 .0 + 1, self.0 .1)),
            Direction::Left => Coordinates((self.0 .0, self.0 .1 - 1)),
            Direction::Right => Coordinates((self.0 .0, self.0 .1 + 1)),
        }
    }
}

fn parse_grid(input: &str) -> (Array2<Tile>, (usize, usize)) {
    let mut lines = input.lines();
    let width = lines.next().unwrap().len() * 2;
    let height = lines.count() + 1;
    let mut bot_position = None;
    let mut raw_tiles = Vec::with_capacity(width * height);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    raw_tiles.push(Tile::Empty);
                    raw_tiles.push(Tile::Empty);
                }
                '#' => {
                    raw_tiles.push(Tile::Immovable);
                    raw_tiles.push(Tile::Immovable);
                }
                '@' => {
                    bot_position = Some((y, x * 2));
                    raw_tiles.push(Tile::Empty);
                    raw_tiles.push(Tile::Empty);
                }
                'O' => {
                    raw_tiles.push(Tile::CrateLeft);
                    raw_tiles.push(Tile::CrateRight);
                }
                _ => panic!("Invalid character in grid: {c}"),
            }
        }
    }
    let grid = Array2::from_shape_vec((height, width), raw_tiles).unwrap();
    (grid, bot_position.unwrap())
}

fn parse_commands(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            '\n' | '\r' => None,
            _ => panic!("Invalid command: {c}"),
        })
        .collect()
}

fn parse_input(input: &str) -> (Array2<Tile>, (usize, usize), Vec<Direction>) {
    let (grid, commands) = input.split_once("\n\n").unwrap();
    let (grid, bot_position) = parse_grid(grid);
    let commands = parse_commands(commands);
    (grid, bot_position, commands)
}

fn flip_crates(grid: &mut Array2<Tile>, from: Coordinates, to: Coordinates) {
    let (y_from, x_from) = from.0;
    let (y_to, x_to) = to.0;
    debug_assert_eq!(y_from, y_to);
    let (x_from, x_to) = if x_from < x_to {
        (x_from, x_to)
    } else {
        (x_to, x_from)
    };
    for x in (x_from + 1)..x_to {
        let tile = grid[(y_from, x)];
        grid[(y_from, x)] = match tile {
            Tile::CrateLeft => Tile::CrateRight,
            Tile::CrateRight => Tile::CrateLeft,
            _ => panic!("Invalid tile: {tile:?}"),
        };
    }
}

fn _print_grid(grid: &Array2<Tile>, bot_position: Coordinates) {
    for (y, row) in grid.outer_iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if (y, x) == bot_position.0 {
                print!("@");
            } else {
                print!(
                    "{}",
                    match tile {
                        Tile::Empty => '.',
                        Tile::CrateLeft => '[',
                        Tile::CrateRight => ']',
                        Tile::Immovable => '#',
                    }
                );    
            }
        }
        println!();
    }
}

fn push_crates(
    grid: &mut Array2<Tile>,
    bot_position: Coordinates,
    new_position: Coordinates,
    command: Direction,
) -> Coordinates {
    match command {
        Direction::Left | Direction::Right => {
            let mut search = new_position;
            loop {
                search = search + command;
                match grid[search.0] {
                    Tile::Immovable => return bot_position,
                    Tile::CrateLeft | Tile::CrateRight => continue,
                    Tile::Empty => {
                        grid[new_position.0] = Tile::Empty;
                        grid[search.0] = if command == Direction::Left {
                            Tile::CrateLeft
                        } else {
                            Tile::CrateRight
                        };
                        flip_crates(grid, search, new_position);
                        return new_position;
                    }
                }
            }
        }
        Direction::Up | Direction::Down => {
            let mut crates_to_move = Vec::new();
            let mut to_check = VecDeque::from(if grid[new_position.0] == Tile::CrateLeft {
                [new_position, new_position + Direction::Right]
            } else {
                [new_position + Direction::Left, new_position]
            });
            while let Some(current) = to_check.pop_front() {
                let next = current + command;
                let current_tile = grid[current.0];
                let next_tile = grid[next.0];
                match (current_tile, next_tile) {
                    (Tile::CrateLeft, Tile::CrateLeft) | (Tile::CrateRight, Tile::CrateRight) => {
                        crates_to_move.push(current);
                        to_check.push_back(next);
                    }
                    (Tile::CrateLeft, Tile::CrateRight) => {
                        crates_to_move.push(current);
                        to_check.push_back(next + Direction::Left);
                        to_check.push_back(next);
                    }
                    (Tile::CrateRight, Tile::CrateLeft) => {
                        crates_to_move.push(current);
                        to_check.push_back(next);
                        to_check.push_back(next + Direction::Right);
                    }
                    (_, Tile::Empty) => {
                        crates_to_move.push(current);
                    }
                    // If we run into a wall anywhere, we can't move the crates
                    (_, Tile::Immovable) => return bot_position,
                    _ => panic!("Invalid tiles: {current_tile:?}, {next_tile:?}"),
                }
            }
            // Also a single crate can be pushed by two crates (which in turn
            // are pushed by one), so the doubling up can happen. Left to right
            // doesn't matter, but the order of the lines does, because so we're
            // not writing over positions we still need to read.
            if command == Direction::Up {
                crates_to_move.sort_unstable_by(|&a, b| a.cmp(b));
            } else {
                crates_to_move.sort_unstable_by(|a, &b| b.cmp(a));
            }
            for from in crates_to_move.into_iter().dedup() {
                let to = from + command;
                debug_assert_eq!(grid[to.0], Tile::Empty);
                grid[to.0] = grid[from.0];
                grid[from.0] = Tile::Empty;
            }
            new_position
        }
    }
}

fn execute_commands(
    mut grid: Array2<Tile>,
    bot_position: (usize, usize),
    commands: &[Direction],
) -> Array2<Tile> {
    let mut bot_position = Coordinates(bot_position);
    for &command in commands {
        let new_position = bot_position + command;
        let bot_tile = grid[bot_position.0];
        debug_assert_eq!(bot_tile, Tile::Empty);
        let new_tile = grid[new_position.0];
        match new_tile {
            Tile::Empty => {
                bot_position = new_position;
            }
            Tile::Immovable => {}
            Tile::CrateLeft | Tile::CrateRight => {
                bot_position = push_crates(&mut grid, bot_position, new_position, command);
            }
        }
    }
    grid
}

fn solve(grid: Array2<Tile>, bot_position: (usize, usize), commands: &[Direction]) -> usize {
    let grid = execute_commands(grid, bot_position, commands);
    grid.indexed_iter()
        .map(
            |((y, x), &tile)| {
                if tile == Tile::CrateLeft {
                    y * 100 + x
                } else {
                    0
                }
            },
        )
        .sum()
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn run(input: &str) -> usize {
    let (grid, bot_position, commands) = parse_input(input);
    solve(grid, bot_position, &commands)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    extern crate test as std_test;
    use super::*;
    use std_test::{black_box, Bencher};
    use test_case::test_case;

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = black_box(&input);
        b.iter(|| parse_input(input));
    }

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let (grid, bot_position, commands) = parse_input(&fs::read_to_string("input.txt").unwrap());
        let grid = black_box(&grid);
        b.iter(|| solve(grid.clone(), bot_position, &commands));
    }

    #[test_case("##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^" => 9021; "big example")]
    fn test(input: &str) -> usize {
        run(input)
    }
}

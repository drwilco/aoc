#![feature(test)]

use std::{fs, ops::Add};

use ndarray::Array2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Crate,
    Immovable,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
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
    let width = lines.next().unwrap().len();
    let height = lines.count() + 1;
    let mut bot_position = None;
    let mut raw_tiles = Vec::with_capacity(width * height);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => raw_tiles.push(Tile::Empty),
                '#' => raw_tiles.push(Tile::Immovable),
                '@' => {
                    bot_position = Some((y, x));
                    raw_tiles.push(Tile::Empty);
                }
                'O' => raw_tiles.push(Tile::Crate),
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

fn execute_commands(
    mut grid: Array2<Tile>,
    bot_position: (usize, usize),
    commands: &[Direction],
) -> Array2<Tile> {
    let mut bot_position = Coordinates(bot_position);
    for &command in commands {
        let new_position = bot_position + command;
        let bot_tile = grid[bot_position.0];
        assert_eq!(bot_tile, Tile::Empty);
        let new_tile = grid[new_position.0];
        match new_tile {
            Tile::Empty => {
                bot_position = new_position;
            }
            Tile::Immovable => {}
            Tile::Crate => {
                let mut search = new_position;
                loop {
                    search = search + command;
                    match grid[search.0] {
                        Tile::Immovable => break,
                        Tile::Crate => continue,
                        Tile::Empty => {
                            grid[new_position.0] = Tile::Empty;
                            grid[search.0] = Tile::Crate;
                            bot_position = new_position;
                            break;
                        }
                    }
                }
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
                if tile == Tile::Crate {
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
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^" => 10092; "big example")]
    #[test_case("########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<" => 2028; "small example")]
    fn test(input: &str) -> usize {
        run(input)
    }
}

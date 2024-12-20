#![feature(test)]

use ndarray::Array2;
use std::{
    collections::VecDeque,
    fs,
    ops::{Add, Not},
};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, EnumIter, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    fn turn_cost(&self, other: Self) -> usize {
        if *self == other {
            0
        } else if self.opposite() == other {
            2000
        } else {
            1000
        }
    }
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Tile {
    costs: [Option<usize>; 4],
}

#[derive(Debug, Clone, Copy)]
struct Coordinates((usize, usize));
impl Add<Direction> for Coordinates {
    type Output = Self;

    // Every map has a border of walls, so we don't need to check for out of
    // bounds
    fn add(self, rhs: Direction) -> Self::Output {
        let Coordinates((y, x)) = self;
        match rhs {
            Direction::Up => Coordinates((y - 1, x)),
            Direction::Down => Coordinates((y + 1, x)),
            Direction::Left => Coordinates((y, x - 1)),
            Direction::Right => Coordinates((y, x + 1)),
        }
    }
}

fn parse_input(input: &str) -> (Array2<Option<Tile>>, Coordinates, Coordinates) {
    let mut lines = input.lines();
    let width = lines.next().unwrap().len();
    let height = 1 + lines.count();
    let mut start = None;
    let mut end = None;
    let mut grid = Array2::from_elem((height, width), None);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = Some(Coordinates((y, x)));
                    grid[[y, x]] = Some(Default::default())
                }
                'E' => {
                    end = Some(Coordinates((y, x)));
                    grid[[y, x]] = Some(Default::default())
                }
                '.' => grid[[y, x]] = Some(Default::default()),
                '#' => (),
                _ => panic!("Invalid character in input"),
            }
        }
    }
    (grid, start.unwrap(), end.unwrap())
}

fn solve(grid: &mut Array2<Option<Tile>>, start: Coordinates, end: Coordinates) -> usize {
    let mut queue = VecDeque::new();
    for direction in Direction::iter() {
        queue.push_back((end, 0, direction));
    }
    grid[end.0].unwrap().costs = [Some(0), Some(0), Some(0), Some(0)];
    while let Some((current_pos, current_cost, current_direction)) = queue.pop_front() {
        let neighbors = Direction::iter().filter_map(|direction| {
            let next_pos = current_pos + direction;
            let next_cost = current_cost + 1 + current_direction.turn_cost(direction);
            grid[next_pos.0].map(|_| (next_pos, next_cost, direction))
        }).collect::<Vec<_>>();
        for (neighbor, next_cost, direction) in neighbors {
            let mut tile = grid[neighbor.0].unwrap();
            if tile.costs[direction as usize]
                .map_or(true, |cost| next_cost < cost)
            {
                tile.costs[direction as usize] = Some(next_cost);
                grid[neighbor.0] = Some(tile);
                queue.push_back((neighbor, next_cost, direction));
            }
        }
    }
    Direction::iter()
        .filter_map(|direction| {
            grid[start.0].unwrap().costs[direction as usize]
                .map(|cost| cost + direction.turn_cost(Direction::Left))
        })
        .min()
        .unwrap()
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn run(input: &str) -> usize {
    let (mut grid, start, end) = parse_input(input);
    solve(&mut grid, start, end)
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
        let input = fs::read_to_string("input.txt").unwrap();
        let (grid, start, end) = parse_input(&input);
        let grid = black_box(&grid);
        b.iter(|| {
            let mut grid = grid.clone();
            solve(&mut grid, start, end)
        });
    }


        #[test_case("###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
" => 7036; "small example")]
        #[test_case("#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
" => 11048; "big example")]
    #[test_case("#####
#..E#
#.#.#
#S..#
#####
" => 1004; "most simple example")]

    fn test(input: &str) -> usize {
        run(input)
    }
}

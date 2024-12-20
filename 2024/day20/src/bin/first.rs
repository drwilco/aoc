#![feature(test)]

use ndarray::Array2;
use std::{fs, ops::Add};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, EnumIter, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Unknown,
    Known(i32),
}

fn parse_input(input: &str) -> (Array2<Tile>, Coordinates) {
    let mut lines = input.lines();
    let width = lines.next().unwrap().len();
    let height = 1 + lines.count();
    let mut start = None;
    let mut grid = Array2::from_elem((height, width), Tile::Wall);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = Some(Coordinates((y, x)));
                    grid[[y, x]] = Tile::Known(0);
                }
                'E' | '.' => grid[[y, x]] = Tile::Unknown,
                '#' => (),
                _ => panic!("Invalid character in input"),
            }
        }
    }
    (grid, start.unwrap())
}

fn set_distances(grid: &mut Array2<Tile>, start: Coordinates) {
    let mut tile = Some((start, 0));
    while let Some((coordinates, steps)) = tile {
        tile = Direction::iter().find_map(|direction| {
            let new_coordinates = coordinates + direction;
            if matches!(grid[new_coordinates.0], Tile::Unknown) {
                grid[new_coordinates.0] = Tile::Known(steps + 1);
                Some((new_coordinates, steps + 1))
            } else {
                None
            }
        });
    }
}

fn is_shortcut(a: Tile, b: Tile) -> Option<i32> {
    match (a, b) {
        (Tile::Known(a), Tile::Known(b)) => {
            let diff = (a - b).abs();
            if diff > 2 {
                Some(diff - 2)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_shortcuts(grid: &Array2<Tile>) -> impl Iterator<Item = i32> + use<'_> {
    grid.windows([3, 1])
        .into_iter()
        .map(|window| {
            let a = window[[0, 0]];
            let b = window[[2, 0]];
            (a, b)
        })
        .chain(grid.windows([1, 3]).into_iter().map(|window| {
            let a = window[[0, 0]];
            let b = window[[0, 2]];
            (a, b)
        }))
        .filter_map(|(a, b)| is_shortcut(a, b))
}

fn solve(mut grid: Array2<Tile>, start: Coordinates) -> usize {
    set_distances(&mut grid, start);
    get_shortcuts(&grid).filter(|&x| x >= 100).count()
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn run(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    solve(grid, start)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    extern crate test as std_test;
    use super::*;
    use itertools::Itertools;
    use std_test::{black_box, Bencher};

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = black_box(&input);
        b.iter(|| parse_input(input));
    }

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let (grid, start) = parse_input(&input);
        let grid = black_box(grid);
        let start = black_box(start);
        b.iter(|| solve(grid.clone(), start));
    }

    #[test]
    fn test_shortcuts() {
        let (mut grid, start) = parse_input(
            "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
",
        );
        set_distances(&mut grid, start);
        let counts = get_shortcuts(&grid).counts();
        let counts = counts
            .into_iter()
            .sorted_unstable_by_key(|&(shortcut, _)| shortcut)
            .collect::<Vec<_>>();
        assert_eq!(
            counts,
            vec![
                (2, 14),
                (4, 14),
                (6, 2),
                (8, 4),
                (10, 2),
                (12, 3),
                (20, 1),
                (36, 1),
                (38, 1),
                (40, 1),
                (64, 1),
            ]
        );
    }
}

#![feature(test)]

use itertools::Itertools;
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
struct Offset((isize, isize));
impl Add<Offset> for Coordinates {
    type Output = Option<Self>;

    fn add(self, rhs: Offset) -> Self::Output {
        Some(Coordinates((
            self.0.0.checked_add_signed(rhs.0.0)?,
            self.0.1.checked_add_signed(rhs.0.1)?,
        )))
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

fn is_shortcut(a: Tile, b: Tile, distance: i32) -> Option<i32> {
    match (a, b) {
        (Tile::Known(a), Tile::Known(b)) => {
            // Since we're not using windows, but comparing each tile with every
            // other tile in range, we don't abs() the difference so that we
            // only count each pair once
            let diff = a - b;
            if diff > distance {
                Some(diff - distance)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_shortcuts(grid: &Array2<Tile>) -> impl Iterator<Item = i32> + use<'_> {
    let offsets = (-20_isize..=20)
        .cartesian_product(-20_isize..=20)
        .filter_map(|(y, x)| {
            if y == 0 && x == 0 {
                return None;
            }
            let distance = i32::try_from(y.abs() + x.abs()).unwrap();
            if distance > 20 {
                return None;
            }
            Some((Offset((y, x)), distance))
        })
        .collect_vec();
    grid.indexed_iter().cartesian_product(offsets).filter_map(
        |((coordinates, &a), (offset, distance))| {
            let a_coordinates = Coordinates(coordinates);
            let b_coordinates = (a_coordinates + offset)?;
            let &b = grid.get(b_coordinates.0)?;
            is_shortcut(a, b, distance)
        },
    )
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
        let counts = get_shortcuts(&grid).filter(|&x| x >= 50).counts();
        let counts = counts
            .into_iter()
            .sorted_unstable_by_key(|&(shortcut, _)| shortcut)
            .collect::<Vec<_>>();
        assert_eq!(
            counts,
            vec![
                (50, 32),
                (52, 31),
                (54, 29),
                (56, 39),
                (58, 25),
                (60, 23),
                (62, 20),
                (64, 19),
                (66, 12),
                (68, 14),
                (70, 12),
                (72, 22),
                (74, 4),
                (76, 3),
            ]
        );
    }
}

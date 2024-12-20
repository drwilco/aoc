#![feature(test)]

use ndarray::Array2;
use nom::{
    character::complete::{char, line_ending, u8},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::{collections::VecDeque, fs, ops::Add};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coordinates((usize, usize));

impl Add<Direction> for Coordinates {
    type Output = Option<Self>;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::Up => Some(Self((self.0 .0.checked_sub(1)?, self.0 .1))),
            Direction::Down => Some(Self((self.0 .0 + 1, self.0 .1))),
            Direction::Left => Some(Self((self.0 .0, self.0 .1.checked_sub(1)?))),
            Direction::Right => Some(Self((self.0 .0, self.0 .1 + 1))),
        }
    }
}

fn parse_input(input: &str) -> Vec<(u8, u8)> {
    let result: IResult<_, _> =
        many1(terminated(separated_pair(u8, char(','), u8), line_ending))(input);
    let (input, coordinates) = result.unwrap();
    assert_eq!(input, "");
    coordinates
}

fn draw_grid(coordinates: &[(u8, u8)], size: usize, amount: usize) -> Array2<bool> {
    let mut grid = Array2::from_elem((size, size), true);
    for (x, y) in &coordinates[..amount] {
        grid[[*y as usize, *x as usize]] = false;
    }
    grid
}

fn solve(coordinates: &[(u8, u8)], size: usize, amount: usize) -> usize {
    let mut grid = draw_grid(coordinates, size, amount);
    let end = Coordinates((size - 1, size - 1));
    grid[end.0] = false;
    let mut stack = VecDeque::from([(end, 0)]);
    while let Some((coordinates, steps)) = stack.pop_front() {
        if coordinates == Coordinates((0, 0)) {
            return steps;
        }
        for direction in Direction::iter() {
            if let Some(new_coordinates) = coordinates + direction {
                if grid.get(new_coordinates.0) == Some(&true) {
                    grid[new_coordinates.0] = false;
                    stack.push_back((new_coordinates, steps + 1));
                }
            }
        }
    }
    panic!("No path found");
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn run(input: &str) -> usize {
    let input = parse_input(input);
    solve(&input, 71, 1024)
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
        let input = parse_input(&fs::read_to_string("input.txt").unwrap());
        let input = black_box(&input);
        b.iter(|| solve(input, 71, 12));
    }

    #[test_case(&[(5,4),(4,2),(4,5),(3,0),(2,1),(6,3),(2,4),(1,5),(0,6),(3,3),(2,6),(5,1),(1,2),
                    (5,5),(2,5),(6,5),(1,4),(0,4),(6,4),(1,1),(6,1),(1,0),(0,5),(1,6),(2,0)], 7, 12 => 22)]
    fn test_solve(input: &[(u8, u8)], size: usize, amount: usize) -> usize {
        solve(input, size, amount)
    }
}

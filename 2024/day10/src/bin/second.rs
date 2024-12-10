#![feature(test)]

use itertools::Itertools;
use ndarray::Array2;
use std::{fs, ops::Add};

fn parse_input(input: &str) -> (Array2<u8>, Vec<Coordinates>) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut raw_tiles = Vec::with_capacity(input.len());
    let mut y = 0_usize;
    let mut x = 0_usize;
    let mut nines = Vec::with_capacity(input.len() / 10);
    for char in input.chars() {
        let char = char as u8;
        let height = match char {
            b'0'..=b'8' => char - b'0',
            b'9' => {
                nines.push(Coordinates((y, x)));
                9
            }
            b'\n' => {
                y += 1;
                x = 0;
                continue;
            }
            _ => panic!("Invalid character: {char}"),
        };
        raw_tiles.push(height);
        x += 1;
    }
    (
        Array2::from_shape_vec((height, width), raw_tiles).unwrap(),
        nines,
    )
}

const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinates((usize, usize));

impl Add<(isize, isize)> for Coordinates {
    type Output = Option<Coordinates>;
    fn add(self, other: (isize, isize)) -> Self::Output {
        let (y, x) = self.0;
        let (dy, dx) = other;
        let y = y.checked_add_signed(dy)?;
        let x = x.checked_add_signed(dx)?;
        Some(Coordinates((y, x)))
    }
}

pub fn run(input: &str) -> usize {
    let (grid, previous_coords) = parse_input(input);
    let mut previous_coords = previous_coords
        .into_iter()
        .map(|coords| (coords, 1))
        .collect::<Vec<_>>();
    for current_height in (0..=8).rev() {
        let current_coords = previous_coords
            .into_iter()
            .cartesian_product(NEIGHBORS)
            .filter_map(|((coords, rating), neighbor)| {
                let current_coords = (coords + neighbor)?;
                let height = grid.get(current_coords.0)?;
                if *height == current_height {
                    Some((current_coords, rating))
                } else {
                    None
                }
            })
            // Multiple paths can lead to the same coordinate, so we combine
            // them, but they can lead to the same 9, so dedup
            .sorted_unstable_by_key(|(coords, _)| *coords)
            .coalesce(|(coords1, rating1), (coords2, rating2)| {
                if coords1 == coords2 {
                    Ok((coords1, rating1 + rating2))
                } else {
                    Err(((coords1, rating1), (coords2, rating2)))
                }
            })
            .collect::<Vec<_>>();
        previous_coords = current_coords;
    }
    previous_coords.into_iter().map(|(_, rating)| rating).sum()
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
    fn my_benchmark(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = black_box(&input);
        b.iter(|| run(input));
    }

    #[test_case("89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
" => 81)]
    fn test(input: &str) -> usize {
        run(input)
    }
}

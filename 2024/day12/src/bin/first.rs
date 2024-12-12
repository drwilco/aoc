#![feature(test)]

use std::{fs, ops::Add};

use ndarray::Array2;

#[derive(Clone, Copy, Debug)]
struct Tile {
    crop: u8,
    checked: bool,
    fences: u8,
}

#[derive(Clone, Copy, Debug)]
struct Coordinates((usize, usize));

impl Add<(isize, isize)> for Coordinates {
    type Output = Option<Self>;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Some(Coordinates((
            self.0 .0.checked_add_signed(rhs.0)?,
            self.0 .1.checked_add_signed(rhs.1)?,
        )))
    }
}

fn parse_input(input: &str) -> Array2<u8> {
    let mut iter = input.lines();
    let width = iter.next().unwrap().len();
    let height = 1 + iter.count();
    let mut elements = Vec::with_capacity(input.len());
    for char in input.lines().flat_map(|line| line.chars()) {
        if char.is_ascii_alphabetic() {
            elements.push(char as u8);
        }
    }
    Array2::from_shape_vec((height, width), elements).unwrap()
}

const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn calculate_fences(grid: &Array2<u8>) -> Array2<Tile> {
    let tiles_with_fences = grid
        .indexed_iter()
        .map(|(position, &crop)| {
            let position = Coordinates(position);
            // the iterator chain finds us the number of neighbors that are the
            // same crop, so we _don't_ need fences there. Hence 4 - that number
            let fences = 4 - NEIGHBORS
                .iter()
                .filter_map(|&neighbor| position + neighbor)
                .filter_map(|neighbor| grid.get(neighbor.0))
                .filter(|&&neighbor| neighbor == crop)
                .count() as u8;
            Tile {
                crop,
                checked: false,
                fences,
            }
        })
        .collect::<Vec<_>>();
    Array2::from_shape_vec(grid.dim(), tiles_with_fences).unwrap()
}

fn solve(grid: &Array2<u8>) -> usize {
    let (height, width) = grid.dim();
    let mut tiles_with_fences = calculate_fences(grid);
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            let position = Coordinates((y, x));
            let tile = tiles_with_fences[position.0];
            if tile.checked {
                continue;
            }
            tiles_with_fences[position.0].checked = true;
            let mut area_fences = tile.fences as usize;
            let mut area_tiles = 0;
            let mut stack = vec![position];
            loop {
                let mut new_stack = Vec::with_capacity(stack.len() * 4);
                for &current in &stack {
                    area_tiles += 1;
                    for &neighbor in &NEIGHBORS {
                        let Some(neighbor_position) = current + neighbor else {
                            continue;
                        };
                        let Some(neighbor_tile) = tiles_with_fences.get_mut(neighbor_position.0)
                        else {
                            continue;
                        };
                        if !neighbor_tile.checked && neighbor_tile.crop == tile.crop {
                            neighbor_tile.checked = true;
                            area_fences += neighbor_tile.fences as usize;
                            new_stack.push(neighbor_position);
                        }
                    }
                }
                if new_stack.is_empty() {
                    total += area_tiles * area_fences;
                    break;
                }
                stack = new_stack;
            }
        }
    }
    total
}

#[must_use]
pub fn run(input: &str) -> usize {
    solve(&parse_input(input))
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
        b.iter(|| solve(input));
    }

    #[test_case("AAAA
BBCD
BBCC
EEEC
" => 140)]
    #[test_case("OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
" => 772)]
    #[test_case("RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
" => 1930)]
    fn test(input: &str) -> usize {
        run(input)
    }
    #[test_case("OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
" => vec![
    2, 2, 1, 2, 2,
    2, 4, 2, 4, 2,
    1, 2, 0, 2, 1,
    2, 4, 2, 4, 2,
    2, 2, 1, 2, 2
    ])]
    fn test_fences(input: &str) -> Vec<u8> {
        let crops = parse_input(input);
        calculate_fences(&crops)
            .iter()
            .map(|&tile| tile.fences)
            .collect()
    }
}

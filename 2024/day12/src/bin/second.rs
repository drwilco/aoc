#![feature(test)]

use std::{
    array, fs,
    ops::{Add, BitOr},
};

use ndarray::Array2;

const NEIGHBORS: [((isize, isize), Direction); 4] = [
    // It is essential that the order of these is a rotation, so that we can
    // easily check for right angles
    ((0, 1), Direction::Right),
    ((1, 0), Direction::Down),
    ((0, -1), Direction::Left),
    ((-1, 0), Direction::Up),
];

const CORNERS: [u8; 4] = [
    Direction::Up as u8 | Direction::Left as u8,
    Direction::Up as u8 | Direction::Right as u8,
    Direction::Down as u8 | Direction::Left as u8,
    Direction::Down as u8 | Direction::Right as u8,
];

#[derive(Clone, Copy, Debug)]
struct Tile {
    crop: u8,
    checked: bool,
    fences: u8,
}

impl Tile {
    fn convex_corners(&self) -> usize {
        CORNERS
            .iter()
            .filter(|&&corner| corner & self.fences == corner)
            .count()
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up = 1 << 0,
    Right = 1 << 1,
    Down = 1 << 2,
    Left = 1 << 3,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
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

fn calculate_fences(grid: &Array2<u8>) -> Array2<Tile> {
    let tiles_with_fences = grid
        .indexed_iter()
        .map(|(position, &crop)| {
            let position = Coordinates(position);
            // the iterator chain finds us the number of neighbors that are the
            // same crop, so we _don't_ need fences there.
            let fences = 0b1111
                & !NEIGHBORS
                    .iter()
                    .filter_map(|&(neighbor, direction)| {
                        let neighbor = (position + neighbor)?;
                        let &neighbor = grid.get(neighbor.0)?;
                        if neighbor == crop {
                            Some(direction as u8)
                        } else {
                            None
                        }
                    })
                    .reduce(BitOr::bitor)
                    .unwrap_or(0);
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
            let mut area_tiles = 0;
            // The amount of sides is the same as the amount of corners
            let mut area_corners = 0;
            let mut stack = vec![position];
            loop {
                let mut new_stack = Vec::with_capacity(stack.len() * 4);
                for &current_coords in &stack {
                    if tiles_with_fences[current_coords.0].checked {
                        continue;
                    }
                    tiles_with_fences[current_coords.0].checked = true;
                    let current_tile = tiles_with_fences[current_coords.0];
                    area_tiles += 1;
                    area_corners += current_tile.convex_corners();
                    let mut neighbors_iter =
                        NEIGHBORS
                            .iter()
                            .map(|&(neighbor_offsets, neighbor_direction)| {
                                let neighbor_direction = neighbor_direction as u8;
                                // If the neighbor is in the direction of current tile's fence, skip it
                                if neighbor_direction & current_tile.fences == neighbor_direction {
                                    return None;
                                }
                                let neighbor_position = (current_coords + neighbor_offsets)?;
                                let neighbor_tile = tiles_with_fences.get(neighbor_position.0)?;
                                if neighbor_tile.crop != tile.crop {
                                    return None;
                                }
                                if !neighbor_tile.checked {
                                    new_stack.push(neighbor_position);
                                }
                                Some((*neighbor_tile, neighbor_direction))
                            });
                    let neighbors: [Option<(Tile, u8)>; 4] =
                        array::from_fn(|_| neighbors_iter.next().unwrap());
                    // Now check for concave corners. We check the neighbors at
                    // right angles, and see if they have fences in the
                    // direction of the other neighbor. We only need to check one,
                    // because the fence indicates a different crop in the diagonal
                    // direction between the two.
                    let concave_corners = [(neighbors[0], neighbors[1]),
                        (neighbors[1], neighbors[2]),
                        (neighbors[2], neighbors[3]),
                        (neighbors[3], neighbors[0])]
                        .into_iter()
                        .filter(|&pair| match pair {
                            (Some((_, direction1)), Some((neighbor2, _))) =>
                                (direction1 & neighbor2.fences) == direction1,
                            _ => false,
                        }).count();
                    area_corners += concave_corners;
                }
                if new_stack.is_empty() {
                    total += area_tiles * area_corners;
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

    #[test_case("OO
OO
" => 16)]
    #[test_case("AAAA
BBCD
BBCC
EEEC
" => 80)]
    #[test_case("OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
" => 436)]
    #[test_case("EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
" => 236)]
    #[test_case("AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
" => 368)]
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
" => 1206)]
    fn test(input: &str) -> usize {
        run(input)
    }

    #[test_case("OO
" => vec![
        Direction::Up as u8 | Direction::Left as u8 | Direction::Down as u8,
        Direction::Up as u8 | Direction::Right as u8 | Direction::Down as u8,
    ])]
    #[test_case("OOO
OXO
OOO
" => vec![
        // OOO
        // OXO
        // OOO
        Direction::Up as u8 | Direction::Left as u8, Direction::Up as u8 | Direction::Down as u8, Direction::Up as u8 | Direction::Right as u8,
        Direction::Left as u8 | Direction::Right as u8, 0b1111, Direction::Left as u8 | Direction::Right as u8,
        Direction::Down as u8 | Direction::Left as u8, Direction::Down as u8 | Direction::Up as u8, Direction::Down as u8 | Direction::Right as u8,
    ])]
    fn test_fences(input: &str) -> Vec<u8> {
        let crops = parse_input(input);
        calculate_fences(&crops)
            .iter()
            .map(|&tile| tile.fences)
            .collect()
    }
}

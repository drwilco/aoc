#![feature(test)]

use std::{cell::Cell, fs, rc::Rc};

use ndarray::Array2;

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileType {
    Empty { visited: u8 },
    Obstacle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up = 1 << 0,
    Down = 1 << 1,
    Left = 1 << 2,
    Right = 1 << 3,
}

fn parse_input(input: &str) -> (Array2<TileType>, (usize, usize), Direction) {
    let start = Rc::new(Cell::new(None));
    let mut height = 0;
    let mut width = None;
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            if width.is_none() {
                width = Some(line.len());
            }
            height += 1;
            let start = start.clone();
            line.chars().enumerate().map(move |(x, c)| match c {
                '.' => TileType::Empty { visited: 0 },
                '#' => TileType::Obstacle,
                '^' => {
                    start.set(Some((y, x)));
                    TileType::Empty {
                        visited: Direction::Up as u8,
                    }
                }
                _ => panic!("Invalid character: {c}"),
            })
        })
        .collect::<Vec<_>>();
    let width = width.unwrap();
    let start = start.get().unwrap();
    let grid = Array2::from_shape_vec((height, width), grid).unwrap();
    (grid, start, Direction::Up)
}

fn will_loop(
    grid: &mut Array2<TileType>,
    start_position: (usize, usize),
    start_direction: Direction,
) -> bool {
    let mut current = Some((start_position, start_direction));
    while let Some((position, direction)) = current {
        if let TileType::Empty { ref mut visited } = grid[position] {
            *visited |= direction as u8;
        }
        let (y, x) = position;
        let next_position = match direction {
            Direction::Up => (y.checked_sub(1), Some(x)),
            Direction::Down => (Some(y + 1), Some(x)),
            Direction::Left => (Some(y), x.checked_sub(1)),
            Direction::Right => (Some(y), Some(x + 1)),
        };
        let next_position = match next_position {
            (Some(y), Some(x)) => Some((y, x)),
            _ => None,
        };
        let next_tile =
            next_position.and_then(|(y, x)| grid.get((y, x)).map(|&tile| (tile, (y, x))));
        let next = match next_tile {
            Some((TileType::Empty { visited }, next_position)) => {
                if visited & direction as u8 != 0 {
                    return true;
                }
                Some((next_position, direction))
            },
            Some((TileType::Obstacle, _)) => {
                let new_direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
                Some((position, new_direction))
            }
            None => None,
        };
        current = next;
    }
    false
}

fn run(input: &str) -> usize {
    let (grid, start_position, start_direction) = parse_input(input);
    grid.indexed_iter()
        .filter_map(|(index, &tile)| {
            if index == start_position || tile == TileType::Obstacle {
                return None;
            }
            let mut working_grid = grid.clone();
            working_grid[index] = TileType::Obstacle;
            let loops = will_loop(&mut working_grid, start_position, start_direction);
            if loops {
                Some(())
            } else {
                None
            }
        })
        .count()
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

    #[test_case("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
" => 6)]
    fn test(input: &str) -> usize {
        run(input)
    }

    #[test_case("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
" => ((6, 4), Direction::Up))]
    fn test_parse_input(input: &str) -> ((usize, usize), Direction) {
        let (_, start, direction) = parse_input(input);
        (start, direction)
    }
}

use anyhow::Result;
use std::{fs, ops};

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::East
    }
}

impl ops::AddAssign<Turn> for Direction {
    fn add_assign(&mut self, rhs: Turn) {
        *self = match rhs {
            Turn::Left => match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::East => Direction::North,
            },
            Turn::Right => match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::East => Direction::South,
            },
        };
    }
}

impl ops::Add<Turn> for Direction {
    type Output = Direction;

    fn add(self, rhs: Turn) -> Direction {
        match rhs {
            Turn::Left => match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::East => Direction::North,
            },
            Turn::Right => match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::East => Direction::South,
            },
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Default)]
struct Point {
    x: isize,
    y: isize,
}

struct Ship {
    position: Point,
    heading: Direction,
}

enum Operation {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

fn parse_operations(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|op| {
            let (op, arg) = op.split_at(1);
            let num: usize = arg.parse().unwrap();
            match op {
                "N" => Operation::North(num),
                "S" => Operation::South(num),
                "E" => Operation::East(num),
                "W" => Operation::West(num),
                "L" => Operation::Left(num),
                "R" => Operation::Right(num),
                "F" => Operation::Forward(num),
                _ => panic!("Unknown operation"),
            }
        })
        .collect()
}

fn do_the_thing(input: &str) -> Result<usize> {
    let ops = parse_operations(input);
    let mut ship = Ship {
        position: Point { x: 0, y: 0 },
        heading: Direction::East,
    };
    for mut op in ops {
        if let Operation::Forward(distance) = op {
            op = match ship.heading {
                Direction::North => Operation::North(distance),
                Direction::South => Operation::South(distance),
                Direction::East => Operation::East(distance),
                Direction::West => Operation::West(distance),
            };
        }
        match op {
            Operation::North(distance) => ship.position.y += distance as isize,
            Operation::South(distance) => ship.position.y -= distance as isize,
            Operation::East(distance) => ship.position.x += distance as isize,
            Operation::West(distance) => ship.position.x -= distance as isize,
            Operation::Left(degrees) => {
                for _ in 0..(degrees / 90) {
                    ship.heading += Turn::Left
                }
            }
            Operation::Right(degrees) => {
                for _ in 0..(degrees / 90) {
                    ship.heading += Turn::Right
                }
            }
            Operation::Forward(_) => panic!("Should not be possible"),
        }
    }
    Ok((ship.position.x.abs() + ship.position.y.abs()) as usize)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("F10
N3
F7
R90
F11" => 25)]
    fn first(input: &str) -> usize {
        do_the_thing(&input).unwrap()
    }
}

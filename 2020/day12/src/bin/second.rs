use anyhow::Result;
use std::fs;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Default)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Ship {
    position: Point,
    waypoint: Point,
}

#[derive(Debug)]
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
        waypoint: Point { x: 10, y: 1 },
    };
    for op in ops {
        match op {
            Operation::North(distance) => ship.waypoint.y += distance as isize,
            Operation::South(distance) => ship.waypoint.y -= distance as isize,
            Operation::East(distance) => ship.waypoint.x += distance as isize,
            Operation::West(distance) => ship.waypoint.x -= distance as isize,
            Operation::Left(180) | Operation::Right(180) => {
                ship.waypoint.x = -ship.waypoint.x;
                ship.waypoint.y = -ship.waypoint.y;
            }
            Operation::Left(90) | Operation::Right(270) => {
                ship.waypoint = Point {
                    x: -ship.waypoint.y,
                    y: ship.waypoint.x,
                };
            }
            Operation::Right(90) | Operation::Left(270) => {
                ship.waypoint = Point {
                    x: ship.waypoint.y,
                    y: -ship.waypoint.x,
                };
            }
            Operation::Forward(times) => {
                ship.position.x += ship.waypoint.x * times as isize;
                ship.position.y += ship.waypoint.y * times as isize;
            }
            _ => panic!("Illegal turn? {:?}", op),
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
F11" => 286)]
    fn first(input: &str) -> usize {
        do_the_thing(&input).unwrap()
    }
}

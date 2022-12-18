use std::{fs, ops::Add};

use nom::{
    character::complete::{char, i32 as parse_i32, line_ending},
    multi::many1,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, x) = parse_i32(input)?;
    let (input, _) = char(',')(input)?;
    let (input, y) = parse_i32(input)?;
    let (input, _) = char(',')(input)?;
    let (input, z) = parse_i32(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Point { x, y, z }))
}
fn do_the_thing(input: &str) -> usize {
    let (input, cubes) = many1(parse_point)(input).unwrap();
    assert!(input.is_empty());

    let sides = vec![
        Point { x: 1, y: 0, z: 0 },
        Point { x: -1, y: 0, z: 0 },
        Point { x: 0, y: 1, z: 0 },
        Point { x: 0, y: -1, z: 0 },
        Point { x: 0, y: 0, z: 1 },
        Point { x: 0, y: 0, z: -1 },
    ];

    cubes
        .iter()
        .map(|cube| {
            sides
                .iter()
                .filter(|side| !cubes.contains(&(cube + side)))
                .count()
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
" => 64)]
    #[test_case("1,1,1
2,1,1
" => 10)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

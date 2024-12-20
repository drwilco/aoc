#![feature(test)]

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i64, line_ending},
    multi::many1,
    sequence::separated_pair,
    IResult,
};
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Bot {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn parse_bot(input: &str) -> IResult<&str, Bot> {
    let mut parse_numbers = separated_pair(i64, char(','), i64);
    let (input, _) = tag("p=")(input)?;
    let (input, pos) = parse_numbers(input)?;
    let (input, _) = tag(" v=")(input)?;
    let (input, vel) = parse_numbers(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Bot { pos, vel }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Bot>> {
    many1(parse_bot)(input)
}

fn solve_single_bot(bot: &Bot, dimensions: (i64, i64), iterations: i64) -> (i64, i64) {
    let (x, y) = bot.pos;
    let (vx, vy) = bot.vel;
    let (width, height) = dimensions;
    let mut x = (x + (iterations * vx)) % width;
    let mut y = (y + (iterations * vy)) % height;
    if x < 0 {
        x += width;
    }
    if y < 0 {
        y += height;
    }
    (x % width, y % height)
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn solve(input: &[Bot], dimensions: (i64, i64), iterations: i64) -> usize {
    let middle_x = dimensions.0 / 2;
    let middle_y = dimensions.1 / 2;
    input
        .iter()
        .filter_map(|bot| {
            let (x, y) = solve_single_bot(bot, dimensions, iterations);
            match (x.cmp(&middle_x), y.cmp(&middle_y)) {
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(Quadrant::TopLeft),
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(Quadrant::BottomLeft),
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(Quadrant::TopRight),
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(Quadrant::BottomRight),
                _ => None,
            }
        })
        .counts()
        .values()
        .product()
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn run(input: &str, dimensions: (i64, i64), iterations: i64) -> usize {
    let input = parse_input(input).unwrap().1;
    solve(&input, dimensions, iterations)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input, (101, 103), 100));
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
        let input = parse_input(&fs::read_to_string("input.txt").unwrap())
            .unwrap()
            .1;
        let input = black_box(&input);
        b.iter(|| solve(input, (101, 103), 100));
    }

    #[test_case("p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
", (11, 7), 100 => 12; "example")]
    fn test(input: &str, dimensions: (i64, i64), iterations: i64) -> usize {
        run(input, dimensions, iterations)
    }
}

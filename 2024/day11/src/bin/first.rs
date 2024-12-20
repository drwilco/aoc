#![feature(test)]
// Needed for the chaining proc macro to work
#![recursion_limit = "512"]

use day11_macros::repeat_methods;
use nom::{
    character::complete::{char, line_ending, u64},
    combinator::eof,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};
use std::fs;

fn parse_u64_list(input: &str) -> IResult<&str, Vec<u64>> {
    terminated(separated_list1(char(' '), u64), tuple((line_ending, eof)))(input)
}

fn parse_input(input: &str) -> Vec<u64> {
    let (_, result) = parse_u64_list(input).unwrap();
    result
}

fn replace_stone(stone: u64) -> [Option<u64>; 2] {
    if stone == 0 {
        return [Some(1), None];
    }
    let digits = stone.ilog10() + 1;
    // even number of digits is uneven ilog10
    if digits % 2 == 0 {
        let half = digits / 2;
        let divider = 10u64.pow(half);
        [Some(stone / divider), Some(stone % divider)]
    } else {
        [Some(stone * 2024), None]
    }
}

// Doing this the naive way, for shits and giggles. But mainly because I wanted
// to see if I could get the iterator mapping chaining to work.
//
// And to make it even more silly, I'm using a macro to repeat the chain.
fn solve25(input: &[u64]) -> usize {
    let iter = repeat_methods!(
        (input.iter().copied()).flat_map(replace_stone).flatten(),
        25
    );
    iter.count()
}

fn solve6(input: &[u64]) -> usize {
    let iter = repeat_methods!((input.iter().copied()).flat_map(replace_stone).flatten(), 6);
    iter.count()
}

#[cfg(test)]
fn solve_box_dyn(input: &[u64], blinks: usize) -> usize {
    let mut iter: Box<dyn Iterator<Item = u64>> = Box::new(input.iter().copied());
    for _ in 0..blinks {
        iter = Box::new(iter.flat_map(replace_stone).flatten());
    }
    iter.count()
}

fn solve(input: &[u64], blinks: usize) -> usize {
    match blinks {
        25 => solve25(input),
        6 => solve6(input),
        _ => panic!("Unsupported number of blinks: {blinks}"),
    }
}

#[must_use]
pub fn run(input: &str) -> usize {
    solve(&parse_input(input), 25)
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
        b.iter(|| solve(input, 25));
    }

    #[bench]
    fn bench_solve_box_dyn(b: &mut Bencher) {
        let input = parse_input(&fs::read_to_string("input.txt").unwrap());
        let input = black_box(&input);
        b.iter(|| solve_box_dyn(input, 25));
    }

    #[test_case("125 17
" => vec![125, 17]; "example")]
    fn test_parse(input: &str) -> Vec<u64> {
        parse_input(input)
    }

    #[test_case(&[125, 17], 6 => 22; "example 1")]
    #[test_case(&[125, 17], 25 => 55312; "example 2")]
    fn test_solve(input: &[u64], blinks: usize) -> usize {
        solve(input, blinks)
    }
}

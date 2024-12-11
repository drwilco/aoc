#![feature(test)]

use nom::{
    character::complete::{char, line_ending, u64},
    combinator::eof,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};
use std::{collections::HashMap, fs};

fn parse_u64_list(input: &str) -> IResult<&str, Vec<u64>> {
    terminated(separated_list1(char(' '), u64), tuple((line_ending, eof)))(input)
}

fn parse_input(input: &str) -> Vec<u64> {
    let (_, result) = parse_u64_list(input).unwrap();
    result
}

fn solve_stone(cache: &mut HashMap<(u64, usize), usize>, stone: u64, blinks: usize) -> usize {
    if blinks == 0 {
        return 1;
    }
    if let Some(&result) = cache.get(&(stone, blinks)) {
        return result;
    }
    let result = if stone == 0 {
        solve_stone(cache, 1, blinks - 1)
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let half = digits / 2;
            let divider = 10u64.pow(half);
            solve_stone(cache, stone / divider, blinks - 1) + solve_stone(cache, stone % divider, blinks - 1)
        } else {
            solve_stone(cache, stone * 2024, blinks - 1)
        }
    };
    cache.insert((stone, blinks), result);
    result
}

fn solve(input: &[u64], blinks: usize) -> usize {
    let mut cache = HashMap::new();
    input.iter().map(|&stone| solve_stone(&mut cache, stone, blinks)).sum()
}

#[must_use]
pub fn run(input: &str) -> usize {
    solve(&parse_input(input), 75)
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

    #[test_case("125 17
" => vec![125, 17]; "example")]
    fn test_parse(input: &str) -> Vec<u64> {
        parse_input(input)
    }

    #[test_case(vec![125, 17], 6 => 22; "example 1")]
    #[test_case(vec![125, 17], 25 => 55312; "example 2")]
    fn test_solve(input: Vec<u64>, blinks: usize) -> usize {
        solve(&input, blinks)
    } 
}

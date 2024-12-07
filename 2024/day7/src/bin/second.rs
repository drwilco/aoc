#![feature(test)]

use itertools::{repeat_n, Itertools};
use nom::{
    character::complete::{char, line_ending, u64},
    combinator::eof,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};
use strum::{EnumIter, IntoEnumIterator};
use std::fs;

#[derive(Debug)]
struct Equation {
    result: u64,
    components: Vec<u64>,
}

#[derive(Clone, Copy, Debug, EnumIter)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Equation {
    fn solvable(&self) -> bool {
        assert!(self.components.len() >= 2);
        repeat_n(
            Operation::iter(),
            self.components.len() - 1,
        )
        .multi_cartesian_product()
        .any(|operations| {
            let mut result = self.components[0];
            for (operation, component) in operations.iter().zip(&self.components[1..]) {
                match operation {
                    Operation::Add => result += component,
                    Operation::Multiply => result *= component,
                    Operation::Concatenate => {
                        let digits = (*component).ilog10() + 1;
                        result = result * 10u64.pow(digits) + component;
                    }
                }
                if result > self.result {
                    break;
                }
            }
            result == self.result
        })
    }
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, result) = u64(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, components) = separated_list1(char(' '), u64)(input)?;
    Ok((input, Equation { result, components }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    terminated(many1(terminated(parse_equation, line_ending)), eof)(input)
}

fn run(input: &str) -> u64 {
    let (_, equations) = parse_input(input).unwrap();
    equations
        .iter()
        .filter_map(|equation| equation.solvable().then_some(equation.result))
        .sum()
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

    #[test_case("190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
" => 11387)]
    fn test(input: &str) -> u64 {
        run(input)
    }

    #[test_case("190: 10 19" => true ; "first solvable example")]
    #[test_case("3267: 81 40 27" => true ; "second solvable example")]
    #[test_case("292: 11 6 16 20" => true ; "third solvable example")]
    #[test_case("156: 15 6" => true ; "fourth solvable example")]
    #[test_case("7290: 6 8 6 15" => true ; "fifth unsolvable example")]
    #[test_case("192: 17 8 14" => true ; "sixth unsolvable example")]

    #[test_case("83: 17 5" => false ; "first unsolvable example")]
    #[test_case("161011: 16 10 13" => false ; "second unsolvable example")]
    #[test_case("21037: 9 7 18 13" => false ; "third unsolvable example")]
    fn test_solvable(input: &str) -> bool {
        let (_, equation) = parse_equation(input).unwrap();
        equation.solvable()
    }
}

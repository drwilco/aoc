#![feature(test)]

use derive_more::derive::From;
use nom::{
    character::complete::{char, line_ending, u8},
    combinator::{eof, into},
    error::Error as NomError,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use std::{collections::HashSet, fs};

#[derive(From)]
struct Update(Vec<u8>);

impl Update {
    fn is_valid(&self, pairs: &Pairs) -> bool {
        self.0.is_sorted_by(|&a, &b| {
            pairs.contains(&(a, b))
        })
    }
    fn corrected(mut self, pairs: &Pairs) -> Self {
        self.0.sort_by(|&a, &b| {
            if pairs.contains(&(a, b)) {
                std::cmp::Ordering::Less
            } else if pairs.contains(&(b, a)) {
                std::cmp::Ordering::Greater
            } else {
                panic!("Invalid pair: ({a}, {b})");
            }
        });
        self
    }
    fn middle_as_u64(&self) -> u64 {
        u64::from(self.0[self.0.len() / 2])
    }
}

type Pairs = HashSet<(u8, u8)>;
type Updates = Vec<Update>;

fn parse_page_pair(input: &str) -> IResult<&str, (u8, u8)> {
    terminated(separated_pair(u8, char('|'), u8), line_ending)(input)
}

fn parse_pairs(input: &str) -> IResult<&str, Pairs> {
    many1(parse_page_pair)(input).map(|(input, pairs)| (input, pairs.into_iter().collect()))
}

fn parse_update(input: &str) -> IResult<&str, Update> {
    into(terminated(
        separated_list1(char::<_, NomError<_>>(','), u8),
        line_ending,
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, (Pairs, Updates)> {
    tuple((
        terminated(parse_pairs, line_ending),
        terminated(many1(parse_update), eof),
    ))(input)
}

fn run(input: &str) -> u64 {
    let (_, (pairs, updates)) = parse_input(input).unwrap();
    updates
        .into_iter()
        .filter_map(|update| {
            (!update.is_valid(&pairs)).then(|| update.corrected(&pairs).middle_as_u64())
        })
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

    const PAIRS: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
";

    #[test_case("75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
" => 123)]
    fn test_example(input: &str) -> u64 {
        let input = PAIRS.to_string() + "\n" + input;
        run(&input)
    }

    #[test_case("75,47,61,53,29
" => true)]
    #[test_case("97,61,53,29,13
" => true)]
    #[test_case("75,29,13
" => true)]
    #[test_case("75,97,47,61,53
" => false)]
    #[test_case("61,13,29
" => false)]
    #[test_case("97,13,75,29,47
" => false)]
    fn test_update(input: &str) -> bool {
        let (_, pairs) = parse_pairs(PAIRS).unwrap();
        let (_, update) = parse_update(input).unwrap();
        update.is_valid(&pairs)
    }

    #[test_case(vec![75,97,47,61,53] => vec![97,75,47,61,53]; "first example")]
    #[test_case(vec![61,13,29] => vec![61,29,13]; "second example")]
    #[test_case(vec![97,13,75,29,47] => vec![97,75,47,29,13]; "third example")]
    fn test_corrected(input: Vec<u8>) -> Vec<u8> {
        let (_, pairs) = parse_pairs(PAIRS).unwrap();
        let input = Update(input);
        input.corrected(&pairs).0
    }
}

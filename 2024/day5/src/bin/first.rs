#![feature(test)]

use std::{collections::HashSet, fs};

type Pairs = HashSet<(u8, u8)>;
type Updates = Vec<Vec<u8>>;

use nom::{
    character::complete::{char, line_ending, u8},
    combinator::eof,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

fn parse_page_pair(input: &str) -> IResult<&str, (u8, u8)> {
    terminated(separated_pair(u8, char('|'), u8), line_ending)(input)
}

fn parse_update(input: &str) -> IResult<&str, Vec<u8>> {
    terminated(separated_list1(char(','), u8), line_ending)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Pairs, Updates)> {
    let (input, pairs) = many1(parse_page_pair)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, updates) = many1(parse_update)(input)?;
    let (input, _) = eof(input)?;
    Ok((input, (pairs.into_iter().collect(), updates)))
}

fn run(input: &str) -> u64 {
    let (_, (pairs, updates)) = parse_input(input).unwrap();
    updates
        .into_iter()
        .filter_map(|update| {
            if update
                .windows(2)
                .all(|pair| pairs.contains(&(pair[0], pair[1])))
            {
                // Return the middle element
                Some(u64::from(update[update.len() / 2]))
            } else {
                None
            }
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

    #[test_case("75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
" => 143)]
    #[test_case("75,47,61,53,29
" => 61)]
    #[test_case("97,61,53,29,13
" => 53)]
    #[test_case("75,29,13
" => 29)]
    #[test_case("75,97,47,61,53
" => 0)]
    #[test_case("61,13,29
" => 0)]
    #[test_case("97,13,75,29,47
" => 0)]
    fn test_example(input: &str) -> u64 {
        let input = "47|53
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

"
        .to_string()
            + input;
        run(&input)
    }
}

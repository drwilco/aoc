use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{eof, map};
use nom::multi::many1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;
use std::fs;

type Range = (u64, u64);

fn parse_num(input: &str) -> IResult<&str, u64> {
    map(digit1, |digit_str: &str| digit_str.parse::<u64>().unwrap())(input)
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    tuple((parse_num, preceded(char('-'), parse_num)))(input)
}

fn parse_pair(input: &str) -> IResult<&str, (Range, Range)> {
    terminated(
        tuple((parse_range, preceded(char(','), parse_range))),
        line_ending,
    )(input)
}

fn completely_contains(needle: Range, haystack: Range) -> bool {
    needle.0 >= haystack.0
        && needle.0 <= haystack.1
        && needle.1 >= haystack.0
        && needle.1 <= haystack.1
}

fn do_the_thing(input: &str) -> u64 {
    let (_, pairs): (_, Vec<(Range, Range)>) =
        terminated(many1(parse_pair), eof)(input).unwrap();
    pairs
        .into_iter()
        .map(|pair| -> u64 {
            (completely_contains(pair.0, pair.1) || completely_contains(pair.1, pair.0)) as u64
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

    #[test_case("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
" => 2)]
    fn test(input: &str) -> u64 {
        do_the_thing(&input)
    }
}

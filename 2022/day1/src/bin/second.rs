use nom::branch::alt;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{eof, map};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use std::fs;

fn parse_num(input: &str) -> IResult<&str, u64> {
    map(digit1, |digit_str: &str| digit_str.parse::<u64>().unwrap())(input)
}

// parse an elf with nom, an elf is multiple lines of numbers
fn parse_elf(input: &str) -> IResult<&str, Vec<u64>> {
    many1(terminated(parse_num, line_ending))(input)
}

fn parse_elves(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    many1(terminated(parse_elf, alt((line_ending, eof))))(input)
}

fn do_the_thing(input: &str) -> u64 {
    let (input, elves) = parse_elves(input).expect("parsing failed");
    assert!(input.is_empty());
    let mut sums = elves
        .into_iter()
        .map(|elf| elf.iter().sum::<u64>())
        .collect::<Vec<_>>();
    sums.sort_by(|a, b| b.cmp(a));
    sums[0..3].iter().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
" => 45000)]
    fn first(input: &str) -> u64 {
        do_the_thing(&input)
    }
}

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
    let (_, elves) = parse_elves(input).expect("parsing failed");
    let max_sum = elves
        .iter()
        .fold(None, |acc, elf| {
            let sum = elf.iter().sum();
            match acc {
                None => Some(sum),
                Some(acc_sum) => Some(std::cmp::max(acc_sum, sum)),
            }
        })
        .unwrap();
    max_sum
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
" => 24000)]
    fn first(input: &str) -> u64 {
        do_the_thing(&input)
    }
}

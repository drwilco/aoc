use anyhow::Result;
use nom::{branch::alt, character::complete::char, combinator::recognize, sequence::pair, IResult};
use std::{env, fs};

fn parser(input: &str) -> IResult<&str, &str> {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"))(input)
}

fn do_the_thing(input: &str) -> IResult<&str, usize> {
    let mut lines = input.lines();
    while lines.next().unwrap() != "" {}
    let result = lines
        .filter(|line| match parser(*line) {
            Ok(result) if result == ("", *line) => true,
            _ => false,
        })
        .count();
    Ok(("", result))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input).unwrap().1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn test1_parser(input: &str) -> IResult<&str, &str> {
        include!(concat!(env!("OUT_DIR"), "/test1.rs"))(input)
    }

    #[test_case("aab
aba" => 2)]
    #[test_case("ab
ba" => 0)]
    fn first(input: &str) -> usize {
        input
            .lines()
            .filter(|line| match test1_parser(*line) {
                Ok(result) if result == ("", *line) => true,
                _ => false,
            })
            .count()
    }
}

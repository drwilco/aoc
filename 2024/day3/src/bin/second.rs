use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u64},
    combinator::value,
    IResult,
};
use std::fs;

fn parse_mul(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("mul(")(input)?;
    let (input, a) = u64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, b) = u64(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, a * b))
}

fn parse_do_dont(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("do()")), value(false, tag("don't()"))))(input)
}

fn run(input: &str) -> u64 {
    (0..input.len())
        .fold((0, true), |(acc, enabled), i| {
            if let Ok((_, result)) = parse_mul(&input[i..]) {
                if enabled {
                    (acc + result, enabled)
                } else {
                    (acc, enabled)
                }
            } else if let Ok((_, enabled)) = parse_do_dont(&input[i..]) {
                (acc, enabled)
            } else {
                (acc, enabled)
            }
        })
        .0
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))" => 161)]
    #[test_case("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))" => 48)]
    fn test(input: &str) -> u64 {
        run(input)
    }
}

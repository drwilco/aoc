use nom::{
    bytes::complete::tag,
    character::complete::{char, u64},
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

fn run(input: &str) -> u64 {
    (0..input.len()).fold(0, |acc, i| {
        if let Ok((_, result)) = parse_mul(&input[i..]) {
            acc + result
        } else {
            acc
        }
    })
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
    fn test(input: &str) -> u64 {
        run(input)
    }
}

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::anychar;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::IResult;

use std::{fs, io};

fn parse_num(input: &str) -> IResult<&str, usize> {
    map(digit1, |digit_str: &str| {
        digit_str.parse::<usize>().unwrap()
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, (usize, usize, char, String)> {
    let (input, first) = parse_num(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, second) = parse_num(input)?;
    let (input, _) = space1(input)?;
    let (input, character) = anychar(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, password) = alpha1(input)?;
    Ok((input, (first, second, character, password.to_string())))
}

fn check_password(first: usize, second: usize, character: char, password: &str) -> bool {
    let password = password.as_bytes();
    let character = character as u8;
    let first = password.get(first - 1).unwrap_or(&(' ' as u8));
    let second = password.get(second - 1).unwrap_or(&(' ' as u8));
    (character != *first && character == *second) || (character == *first && character != *second)
}

fn check_passwords(password_list: &str) -> io::Result<usize> {
    Ok(password_list
        .lines()
        .filter(|line| {
            let (_, (first, second, character, password)) = parse_line(&line).unwrap();
            check_password(first, second, character, &password)
        })
        .collect::<Vec<_>>()
        .len())
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", check_passwords(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 3, 'a', "abcde" => true; "line1")]
    #[test_case(1, 3, 'b', "cdefg" => false; "line2")]
    #[test_case(2, 9, 'c', "ccccccccc" => false; "line3")]
    fn first(first: usize, second: usize, character: char, password: &str) -> bool {
        check_password(first, second, character, password)
    }
}

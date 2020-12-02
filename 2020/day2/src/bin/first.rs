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
    let (input, min) = parse_num(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, max) = parse_num(input)?;
    let (input, _) = space1(input)?;
    let (input, character) = anychar(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, password) = alpha1(input)?;
    Ok((input, (min, max, character, password.to_string())))
}

fn check_passwords(password_list: &str) -> io::Result<usize> {
    Ok(password_list
        .lines()
        .filter_map(|line| {
            let (_, (min, max, character, password)) = parse_line(&line).unwrap();
            let count = password
                .chars()
                .filter(|c| *c == character)
                .collect::<Vec<char>>()
                .len();
            if count >= min && count <= max {
                Some(())
            } else {
                None
            }
        })
        .collect::<Vec<()>>()
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
    #[test]
    fn first() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(check_passwords(input).unwrap(), 2);
    }
}

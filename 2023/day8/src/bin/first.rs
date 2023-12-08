use std::{fs, collections::HashMap};

use nom::{IResult, sequence::{terminated, pair}, multi::many1, branch::alt, combinator::map, bytes::complete::{tag, take}, character::complete::line_ending};

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    terminated(
        many1(
            alt((
                map(tag("L"), |_| Instruction::Left),
                map(tag("R"), |_| Instruction::Right),
            )
        )),
        pair(line_ending, line_ending)
    )(input)
}

#[derive(Debug)]
struct Choice<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_choice(input: &str) -> IResult<&str, Choice> {
    let (input, name) = take(3_usize)(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = take(3_usize)(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = take(3_usize)(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Choice { name, left, right }))
}

fn parse_choices(input: &str) -> IResult<&str, HashMap<&str, Choice>> {
    let (input, choices) = many1(parse_choice)(input)?;
    Ok((input, HashMap::from_iter(choices.into_iter().map(|choice| (choice.name, choice)))))
}

pub fn run(input: &str) -> i64 {
    let (input, instructions) = parse_instructions(input).unwrap();
    let (input, choices) = parse_choices(input).unwrap();
    assert!(input.is_empty());
    let mut instructions = instructions.iter().cycle();
    let mut current = "AAA";
    let mut count = 0;
    while current != "ZZZ" {
        count += 1;
        let instruction = instructions.next().unwrap();
        current = match instruction {
            Instruction::Left => choices.get(current).unwrap().left,
            Instruction::Right => choices.get(current).unwrap().right,
        };
    }
    count
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
" => 2)]
    #[test_case("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
" => 6)]
    fn test(input: &str) -> i64 {
        run(&input)
    }
}

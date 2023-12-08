use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::line_ending,
    combinator::map,
    multi::many1,
    sequence::{pair, terminated},
    IResult,
};

use num::Integer;

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    terminated(
        many1(alt((
            map(tag("L"), |_| Instruction::Left),
            map(tag("R"), |_| Instruction::Right),
        ))),
        pair(line_ending, line_ending),
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
    Ok((
        input,
        HashMap::from_iter(choices.into_iter().map(|choice| (choice.name, choice))),
    ))
}

fn find_cycle_length(
    choices: &HashMap<&str, Choice>,
    start: &str,
    instructions: &[Instruction],
) -> i64 {
    let mut instructions = instructions.iter().cycle();
    let mut current = start;
    let mut count = 0;
    while !current.ends_with('Z') {
        count += 1;
        let instruction = instructions.next().unwrap();
        current = match instruction {
            Instruction::Left => choices.get(current).unwrap().left,
            Instruction::Right => choices.get(current).unwrap().right,
        };
    }
    count
}

pub fn run(input: &str) -> i64 {
    let (input, instructions) = parse_instructions(input).unwrap();
    let (input, choices) = parse_choices(input).unwrap();
    assert!(input.is_empty());
    choices
        .keys()
        .filter_map(|place| {
            if place.ends_with('A') {
                Some(find_cycle_length(&choices, place, &instructions))
            } else {
                None
            }
        })
        .fold(1, |acc, len| acc.lcm(&len))
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
" => 6)]
    fn test(input: &str) -> i64 {
        run(&input)
    }
}

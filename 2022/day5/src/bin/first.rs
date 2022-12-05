use std::fs;

use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{map, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, terminated, tuple};
use nom::{IResult, Parser};

#[derive(Debug, PartialEq)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn number(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = number(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = number(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = number(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Instruction { amount, from, to }))
}

fn parse_container(input: &str) -> IResult<&str, Option<char>> {
    alt((
        value(None, tag("   ")),
        preceded(
            char('['),
            terminated(
                is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ").map(|s: &str| {
                    assert!(s.len() == 1);
                    Some(s.chars().next().unwrap())
                }),
                char(']'),
            ),
        ),
    ))(input)
}

fn parse_layer(input: &str) -> IResult<&str, Vec<Option<char>>> {
    terminated(separated_list1(char(' '), parse_container), line_ending)(input)
}

fn parse_label(input: &str) -> IResult<&str, usize> {
    preceded(char(' '), terminated(number, char(' ')))(input)
}

fn parse_label_line(input: &str) -> IResult<&str, Vec<usize>> {
    terminated(
        separated_list1(char(' '), parse_label),
        tuple((line_ending, line_ending)),
    )(input)
}

fn do_the_thing(input: &str) -> String {
    let (input, mut layers) = many1(parse_layer)(input).unwrap();
    let (input, labels) = parse_label_line(input).unwrap();
    let (input, instructions) = many1(parse_instruction)(input).unwrap();

    assert!(input.is_empty());
    // if there's 4 labels, the last one should be 4
    assert_eq!(labels.len(), *labels.last().unwrap());
    // non-exhaustive check that layers are the same length as labels
    assert_eq!(labels.len(), layers[0].len());
    // pivot layers into stacks
    let mut stacks: Vec<Vec<char>> = labels.iter().map(|_| Vec::new()).collect();
    layers.reverse();
    for layer in layers {
        for (stack, container) in stacks.iter_mut().zip(layer) {
            if let Some(c) = container {
                stack.push(c);
            }
        }
    }
    // execute instructions
    for instruction in instructions {
        for _ in 0..instruction.amount {
            let c = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(c);
        }
    }
    // read top of the stacks
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
" => "CMZ")]
    fn everything(input: &str) -> String {
        do_the_thing(&input)
    }

    #[test_case("move 1 from 2 to 1\n" => Ok(("", Instruction { amount: 1, from: 2, to: 1 })))]
    fn instruction(input: &str) -> IResult<&str, Instruction> {
        parse_instruction(&input)
    }

    #[test_case("   " => Ok(("", None)))]
    #[test_case("[A]" => Ok(("", Some('A'))))]
    fn container(input: &str) -> IResult<&str, Option<char>> {
        parse_container(&input)
    }

    #[test_case("    [A] [B] [C]\n" => Ok(("", vec![None, Some('A'), Some('B'), Some('C')])))]
    fn layer(input: &str) -> IResult<&str, Vec<Option<char>>> {
        parse_layer(&input)
    }

    #[test_case(" 1   2   3 \n\n" => Ok(("", vec![1, 2, 3])))]
    fn label_line(input: &str) -> IResult<&str, Vec<usize>> {
        parse_label_line(&input)
    }
}

use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    AddX(isize),
    NoOp,
}

struct State {
    x: isize,
    cycle: usize,
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, value},
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, isize> {
    alt((
        map(digit1, |digit_str: &str| {
            digit_str.parse::<isize>().unwrap()
        }),
        map(preceded(tag("-"), digit1), |digit_str: &str| {
            -digit_str.parse::<isize>().unwrap()
        }),
    ))(input)
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::NoOp, tag("noop"))(input)
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("addx "), parse_number), Instruction::AddX)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    terminated(alt((parse_noop, parse_addx)), line_ending)(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(parse_instruction)(input)
}

fn instructions_to_cycles(instructions: Vec<Instruction>) -> Vec<isize> {
    let mut state = State { x: 1, cycle: 0 };
    instructions
        .into_iter()
        .flat_map(|instruction| match instruction {
            Instruction::AddX(n) => {
                let cycles = vec![state.x, state.x];
                state.x += n;
                state.cycle += 1;
                cycles.into_iter()
            }
            Instruction::NoOp => {
                let cycles = vec![state.x];
                state.cycle += 1;
                cycles.into_iter()
            }
        })
        .collect()
}

fn do_the_thing(input: &str) -> isize {
    let (input, instructions) = parse_instructions(input).unwrap();
    assert!(input.is_empty());
    let cycles = instructions_to_cycles(instructions);
    let mut cycles = cycles.into_iter();
    let mut signal_strength = cycles.nth(19).unwrap() * 20;
    for i in 1..=5 {
        signal_strength += cycles.nth(39).unwrap() * (i * 40 + 20);
    }
    signal_strength
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test_case(INPUT => 13140)]
    fn test(input: &str) -> isize {
        do_the_thing(&input)
    }

    #[test]
    fn test2() {
        let (input, instructions) = parse_instructions(INPUT).unwrap();
        assert!(input.is_empty());
        let cycles = instructions_to_cycles(instructions);
        assert_eq!(cycles[19], 21);
        assert_eq!(cycles[59], 19);
        assert_eq!(cycles[99], 18);
        assert_eq!(cycles[139], 21);
        assert_eq!(cycles[179], 16);
        assert_eq!(cycles[219], 18);
    }
}

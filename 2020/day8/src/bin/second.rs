use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, one_of, space1},
    combinator::{map, opt, recognize, value},
    multi::separated_list1,
    sequence::pair,
    IResult,
};
use std::{cell::Cell, fs, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operation {
    ACC,
    JMP,
    NOP,
}

use Operation::*;

#[derive(Clone, Debug)]
struct Instruction {
    operation: Cell<Operation>,
    argument: i128,
    use_count: usize,
}

fn parse_num<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    map(
        recognize(pair(opt(one_of("+-")), digit1)),
        |digit_str: &str| digit_str.parse::<T>().unwrap(),
    )(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, operation) = alt((
        value(ACC, tag("acc")),
        value(JMP, tag("jmp")),
        value(NOP, tag("nop")),
    ))(input)?;
    let (input, _) = space1(input)?;
    let (input, argument) = parse_num(input)?;
    Ok((
        input,
        Instruction {
            operation: Cell::new(operation),
            argument,
            use_count: 0,
        },
    ))
}

fn run_program(instructions: &mut [Instruction]) -> Result<i128> {
    let mut ip: isize = 0;
    let mut acc = 0;
    loop {
        if ip == instructions.len() as isize {
            return Ok(acc);
        } else if ip > instructions.len() as isize {
            return Err(anyhow!("instruction pointer out of bounds"));
        }
        let instruction = &mut instructions[ip as usize];
        if instruction.use_count == 1 {
            return Err(anyhow!("loop detected"));
        }
        match instruction.operation.get() {
            ACC => {
                acc += instruction.argument;
                ip += 1;
            }
            JMP => {
                ip += instruction.argument as isize;
            }
            NOP => {
                ip += 1;
            }
        }
        instruction.use_count += 1;
    }
}

fn do_the_thing(input: &str) -> i128 {
    let (_, instructions) = separated_list1(line_ending, parse_instruction)(input).unwrap();

    instructions
        .iter()
        .rev()
        .find_map(|instruction| match instruction.operation.get() {
            ACC => None,
            JMP | NOP => {
                let original = instruction.operation.get();
                if original == JMP {
                    instruction.operation.set(NOP);
                } else {
                    instruction.operation.set(JMP);
                }
                let mut clone = instructions.clone();
                instruction.operation.set(original);
                run_program(&mut clone).ok()
            }
        })
        .unwrap()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6" => 8)]
    fn first(input: &str) -> i128 {
        do_the_thing(&input)
    }
}

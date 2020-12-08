use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, one_of, space1},
    combinator::{map, opt, recognize, value},
    multi::separated_list1,
    sequence::pair,
    IResult,
};
use std::{fs, io, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operation {
    ACC,
    JMP,
    NOP,
}

use Operation::*;

#[derive(Clone, Debug)]
struct Instruction {
    operation: Operation,
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
            operation,
            argument,
            use_count: 0,
        },
    ))
}

fn run_program(instructions: &mut [Instruction]) -> std::result::Result<i128, Vec<usize>> {
    let mut ip: usize = 0;
    let mut path = Vec::<usize>::new();
    let mut acc = 0;
    loop {
        let instruction = &mut instructions[ip];
        if instruction.use_count == 1 {
            return Err(path);
        }
        path.push(ip);
        match instruction.operation {
            ACC => {
                acc += instruction.argument;
                ip += 1;
            }
            JMP => {
                ip = (ip as i128 + instruction.argument) as usize;
            }
            NOP => {
                ip += 1;
            }
        }
        instruction.use_count += 1;
        if ip == instructions.len() {
            return Ok(acc);
        }
    }
}

fn do_the_thing(input: &str) -> i128 {
    let (_, instructions) = separated_list1(line_ending, parse_instruction)(input).unwrap();
    let result = run_program(&mut (instructions.clone()));
    if let Err(bad_instructions) = result {
        println!("{}", bad_instructions.len());
        bad_instructions.into_iter().rev().find_map(|bad_instruction| {
            let mut instructions = instructions.clone();
            match instructions[bad_instruction].operation {
                NOP => instructions[bad_instruction].operation = JMP,
                JMP => instructions[bad_instruction].operation = NOP,
                ACC => (),
            }
            run_program(&mut instructions).ok()
        }).unwrap()
    } else {
        result.unwrap()
    }
}

fn main() -> io::Result<()> {
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

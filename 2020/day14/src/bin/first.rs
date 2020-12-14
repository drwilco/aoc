use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{digit1, line_ending, one_of},
    combinator::{map, opt, recognize},
    multi::separated_list1,
    sequence::pair,
    IResult,
};
use std::{collections::HashMap, fs, str::FromStr};

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

#[derive(Debug)]
struct Mem {
    address: u64,
    value: u64,
}

#[derive(Debug)]
struct Mask {
    and: u64,
    or: u64,
}

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    Mem(Mem),
}

fn parse_mem(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mem[")(input)?;
    let (input, address) = parse_num(input)?;
    let (input, _) = tag("] = ")(input)?;
    let (input, value) = parse_num(input)?;
    Ok((input, Instruction::Mem(Mem { address, value })))
}

fn parse_mask(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mask = ")(input)?;
    let (input, mask) = is_a("10X")(input)?;
    // To force zeros, we have an and_mask with 1s everywhere except where a 0 is
    // in the input mask. To force ones, we have an or_mask with 1s where there is
    // a 1 in the input mask:
    // input:    XXX0XXX1XX
    // and_mask: 1110111111
    // or_mask:  0000000100
    let (and, or) = mask.chars().fold((0, 0), |(mut and_mask, mut or_mask), c| {
        and_mask <<= 1;
        or_mask <<= 1;
        match c {
            '1' => {
                or_mask |= 1;
                and_mask |= 1;
            }
            '0' => (),
            'X' => and_mask |= 1,
            _ => panic!("invalid mask input"),
        }
        (and_mask, or_mask)
    });
    Ok((input, Instruction::Mask(Mask { and, or })))
}

fn do_the_thing(input: &str) -> u64 {
    let (_, instructions) =
        separated_list1(line_ending, alt((parse_mask, parse_mem)))(input).unwrap();

    let (results, _) = instructions.into_iter().fold(
        (HashMap::new(), None),
        |(mut acc, current_mask), instruction| match instruction {
            Instruction::Mask(mask) => (acc, Some(mask)),
            Instruction::Mem(mem) => {
                let mask = current_mask.as_ref().unwrap();
                let mut value = mem.value | mask.or;
                value &= mask.and;
                acc.insert(mem.address, value);
                (acc, current_mask)
            }
        },
    );
    results.values().sum()
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

    #[test_case("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0" => 165)]
    fn first(input: &str) -> u64 {
        do_the_thing(&input)
    }
}

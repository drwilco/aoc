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

#[derive(Debug, Default)]
struct Mask {
    or: u64,
    xors: Vec<u64>,
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
    // Every 1 forces a 1, every 0 leaves the original unchanged, that's
    // the same as binary or.
    // Then every X causes both, so make a list of masks to use to toggle.
    assert_eq!(mask.len(), 36);
    Ok((
        input,
        Instruction::Mask(mask.chars().fold(Mask::default(), |mut mask, c| {
            mask.or <<= 1;
            for xor in &mut mask.xors {
                *xor <<= 1;
            }
            match c {
                '1' => mask.or |= 1,
                '0' => (),
                'X' => mask.xors.push(1),
                _ => panic!("impossible"),
            }
            mask
        })),
    ))
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
                let addresses: Vec<_> =
                    mask.xors
                        .iter()
                        .fold(vec![mem.address | mask.or], |acc, xor| {
                            acc.into_iter()
                                .flat_map(|address| vec![address, address ^ xor].into_iter())
                                .collect()
                        });
                for address in addresses {
                    acc.insert(address, mem.value);
                }
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

    #[test_case("mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1" => 208)]
    fn first(input: &str) -> u64 {
        do_the_thing(&input)
    }
}

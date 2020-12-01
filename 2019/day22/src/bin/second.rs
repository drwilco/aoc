use std::io;
use std::fs;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;
//use std::time::Instant;

#[derive(Debug)]
enum Instruction {
    DealIntoNew,
    Cut(i64),
    DealWithIncr(u64),
}

fn deal_into_new(pos: u64, max: u64) -> u64 {
    max - pos
}

fn cut(pos: u64, size: u64, index: i64) -> u64 {
    let index: u64 = if index < 0 {
        (index + (size as i64)) as u64
    } else {
        index as u64
    };
    if pos < index {
        pos + (size - index)
    } else {
        pos - index
    }
}

fn deal_with_incr(pos: u64, size: u64, increment: u64) -> u64 {
    (pos * increment) % size
}

fn parse_num(input: &str) -> IResult<&str, i64> {
  alt((
    map(digit1, |digit_str: &str| digit_str.parse::<i64>().unwrap()),
    map(preceded(tag("-"), digit1), |digit_str: &str| 
      -1 * digit_str.parse::<i64>().unwrap()),
  ))(input)
}

fn parse_new(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("deal into new stack")(input)?;
    Ok((input, Instruction::DealIntoNew))
}

fn parse_cut(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("cut ")(input)?;
    let (input, index) = parse_num(input)?;
    Ok((input, Instruction::Cut(index)))
}

fn parse_deal(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("deal with increment ")(input)?;
    let (input, incr) = parse_num(input)?;
    Ok((input, Instruction::DealWithIncr(incr as u64)))
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    for line in instructions.lines() {
        let (_, instr) = alt((parse_new, parse_cut, parse_deal))(line).unwrap();
        result.push(instr);
    }
    result
}

fn apply_instructions(mut pos: u64, size: u64, instructions: &Vec<Instruction>) -> u64 {
    let max = size - 1;
    for instr in instructions {
        match *instr {
            Instruction::DealIntoNew => pos = deal_into_new(pos, max),
            Instruction::Cut(index) => pos = cut(pos, size, index),
            Instruction::DealWithIncr(incr) => pos = deal_with_incr(pos, size, incr),
        }
    }
    pos
}

fn main() -> io::Result<()> {
    let instructions = fs::read_to_string("input.txt")?;
    let size = 119315717514047;
    let instructions = parse_instructions(&instructions);
    let mut pos = apply_instructions(2020, size, &instructions);
    println!("pos: {:?}", pos);
    let mut reps: u64 = 1;
//    let start = Instant::now();
    while pos != 2020 && reps < 101_741_582_076_661 {
        pos = apply_instructions(pos, size, &instructions);
        reps += 1;
//        if reps % 1_000_000 == 0 {
//            println!("{}: {:?}", reps, start.elapsed());
//        }
        println!("pos: {:?}", pos);
    }
    println!("pos is {} after {} reps", pos, reps);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test_case(10, 0 => 9; "example 1 0")]
    #[test_case(10, 1 => 8; "example 1 1")]
    #[test_case(10, 4 => 5; "example 1 4")]
    #[test_case(10, 5 => 4; "example 1 5")]
    #[test_case(10, 8 => 1; "example 1 8")]
    #[test_case(10, 9 => 0; "example 1 9")]
    fn test_new(size: u64, card: u64) -> u64 {
        let max = size - 1;
        deal_into_new(card, max)
    }

    #[test_case(10, 3 => vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]; "example 2")]
    #[test_case(10, -4 => vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]; "example 3")]
    fn test_cut(size: u64, index: i64) -> Vec<u64> {
        let mut result = Vec::new();
        result.resize(10, 0);
        for card in 0..size {
            result[cut(card, size, index) as usize] = card;
        }
        result
    }

    #[test]
    fn example_4() {
        let mut result = Vec::new();
        let increment = 3;
        let size = 10;
        result.resize(10, 0);
        for card in 0..size {
            result[deal_with_incr(card, size, increment) as usize] = card;
        }
        assert_eq!(result, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test_case(10, "deal with increment 7
deal into new stack
deal into new stack" => vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]; "example 5")]
    #[test_case(10, "cut 6
deal with increment 7
deal into new stack" => vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]; "example 6")]
    #[test_case(10, "deal with increment 7
deal with increment 9
cut -2" => vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]; "example 7")]
    #[test_case(10, "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1" => vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]; "example 8")]
    fn test_instructions(size: u64, instructions: &str) -> Vec<u64> {
        let mut result = Vec::new();
        result.resize(10, 0);
        let instructions = parse_instructions(&instructions);
        for card in 0..size {
            result[apply_instructions(card, size, &instructions) as usize] = card;
        }
        result
    }

    #[test]
    fn part_1() {
        let instructions = fs::read_to_string("input.txt").expect("can't read file");
        let instructions = parse_instructions(&instructions);
        let pos = apply_instructions(2019, 10007, &instructions);
        assert_eq!(pos, 3324);
    }
}


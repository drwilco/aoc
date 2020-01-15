use std::io;
use std::fs;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

type Deck = Vec<u32>;

#[derive(Debug)]
enum Instruction {
    DealIntoNew,
    Cut(isize),
    DealWithIncr(usize),
}

fn deal_into_new(mut deck: Deck) -> Deck {
    deck.reverse();
    deck
}

fn cut(mut deck: Deck, mut index: isize) -> Deck {
    if index < 0 {
        index += deck.len() as isize;
    }
    let mut new_deck: Deck = Vec::with_capacity(deck.len());
    let mut bottom = deck.split_off(index as usize);
    new_deck.append(&mut bottom);
    new_deck.append(&mut deck);
    new_deck
}

fn deal_with_incr(deck: Deck, increment: usize) -> Deck {
    let len = deck.len();
    let mut new_deck: Deck = Vec::with_capacity(len);
    new_deck.resize(len, 0);
    for (pos, card) in deck.into_iter().enumerate() {
        let index = (pos * increment) % len;
        new_deck[index] = card;
    }
    new_deck
}

fn make_deck(size: usize) -> Deck {
    let mut deck: Deck = Vec::with_capacity(size);
    for c in 0..size {
        deck.push(c as u32)
    }
    deck
}

fn parse_num(input: &str) -> IResult<&str, isize> {
  alt((
    map(digit1, |digit_str: &str| digit_str.parse::<isize>().unwrap()),
    map(preceded(tag("-"), digit1), |digit_str: &str| 
      -1 * digit_str.parse::<isize>().unwrap()),
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
    Ok((input, Instruction::DealWithIncr(incr as usize)))
}


fn apply_instructions(mut deck: Deck, instructions: &str) -> Deck {
    for line in instructions.lines() {
        let (_, instr) = alt((parse_new, parse_cut, parse_deal))(line).unwrap();
        match instr {
            Instruction::DealIntoNew => deck = deal_into_new(deck),
            Instruction::Cut(index) => deck = cut(deck, index),
            Instruction::DealWithIncr(incr) => deck = deal_with_incr(deck, incr),
        }
    }
    deck
}

fn main() -> io::Result<()> {
    let deck = make_deck(10007);
    let instructions = fs::read_to_string("input.txt")?;
    let deck = apply_instructions(deck, &instructions);
    println!("index of 2019: {}", deck.iter().position(|&card| card == 2019).unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test]
    fn example_1() {
        let deck = make_deck(10);
        assert_eq!(deal_into_new(deck), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test_case(10, 3 => vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]; "example 2")]
    #[test_case(10, -4 => vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]; "example 3")]
    fn test_cut(size: usize, index: isize) -> Deck {
        let deck = make_deck(size);
        cut(deck, index)
    }

    #[test]
    fn example_4() {
        assert_eq!(deal_with_incr(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3), vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
        
    }

    #[test_case(10, "deal with increment 7
deal into new stack
deal into new stack" => vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]; "example 5")]
    fn test_instructions(size: usize, instructions: &str) -> Deck {
        let deck = make_deck(size);
        apply_instructions(deck, instructions)
    }

}


use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
    IResult,
};
use std::{collections::VecDeque, fs, str::FromStr};

fn parse_num<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    map(digit1, |digit_str: &str| digit_str.parse::<T>().unwrap())(input)
}

fn parse_deck(input: &str) -> IResult<&str, VecDeque<usize>> {
    let (input, _) = tag("Player ")(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, deck) = separated_list1(line_ending, parse_num)(input)?;
    Ok((input, deck.into()))
}

fn do_the_thing(input: &str) -> IResult<&str, usize> {
    let (input, mut decks): (&str, Vec<VecDeque<usize>>) =
        separated_list1(pair(line_ending, line_ending), parse_deck)(input)?;
    while decks.len() > 1 {
        let mut round_cards = decks
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect::<Vec<_>>();
        let round_winner = round_cards
            .iter()
            .enumerate()
            .fold((0, 0), |(highest, winner), (index, card)| {
                if *card > highest {
                    (*card, index)
                } else {
                    (highest, winner)
                }
            })
            .1;
        round_cards.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        decks[round_winner].extend(round_cards);
        decks = decks.into_iter().filter(|deck| !deck.is_empty()).collect();
    }
    let result = decks[0]
        .iter()
        .rev()
        .enumerate()
        .map(|(index, card)| (index + 1) * card)
        .sum();
    Ok((input, result))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input).unwrap().1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10" => 306)]
    fn first(input: &str) -> usize {
        do_the_thing(&input).unwrap().1
    }
}

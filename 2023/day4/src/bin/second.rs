use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Game {
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
    amount: usize,
}

impl Game {
    fn score(&self) -> usize {
        let winning_numbers: HashSet<usize> =
            HashSet::from_iter(self.winning_numbers.iter().cloned());
        let numbers: HashSet<usize> = HashSet::from_iter(self.numbers.iter().cloned());
        numbers.intersection(&winning_numbers).count()
    }
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, parse_number)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space1(input)?;
    let (input, winning_numbers) = parse_numbers(input)?;
    let (input, _) = tuple((space1, char('|'), space1))(input)?;
    let (input, numbers) = parse_numbers(input)?;
    let (input, _) = line_ending(input)?;
    Ok((
        input,
        Game {
            winning_numbers,
            numbers,
            amount: 1,
        },
    ))
}

fn do_the_thing(input: &str) -> usize {
    let (input, mut games) = many1(parse_game)(input).unwrap();
    assert!(input.is_empty());
    for i in 0..games.len() {
        let score = games[i].score();
        let amount = games[i].amount;
        for j in 0..score {
            games[i+j+1].amount += amount;
        }
    }
    games.into_iter().map(|g| g.amount).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
" => 30)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
    IResult,
};
use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    fs,
    str::FromStr,
};

fn parse_num<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    map(digit1, |digit_str: &str| digit_str.parse::<T>().unwrap())(input)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Deck {
    player: usize,
    cards: VecDeque<usize>,
}

impl Deck {
    fn take_top(&mut self) -> usize {
        self.cards.pop_front().unwrap()
    }
    fn add_to_bottom(&mut self, cards: Vec<(usize, usize)>) {
        for (_, card) in cards {
            self.cards.push_back(card);
        }
    }
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    fn score(&self) -> usize {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .map(|(index, card)| (index + 1) * card)
            .sum()
    }
    fn parse(input: &str) -> IResult<&str, Deck> {
        let (input, _) = tag("Player ")(input)?;
        let (input, player) = parse_num(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = line_ending(input)?;
        let (input, cards) = separated_list1(line_ending, parse_num)(input)?;
        Ok((
            input,
            Deck {
                player,
                cards: cards.into(),
            },
        ))
    }
}

type RoundsSet = HashSet<Vec<Deck>>;

fn play_the_game(mut decks: Vec<Deck>) -> Deck {
    let mut previous_rounds: RoundsSet = HashSet::new();

    while decks.len() > 1 {
        if !previous_rounds.insert(decks.clone()) {
            return decks.into_iter().find(|deck| deck.player == 1).unwrap();
        }
        let mut round_cards = decks
            .iter_mut()
            .map(|deck| (deck.player, deck.take_top()))
            .collect::<Vec<_>>();
        let enough_cards = round_cards
            .iter()
            .zip(decks.iter())
            .find(|((_, card), deck)| deck.cards.len() < *card)
            .is_none();
        let round_winner = if !enough_cards {
            // not enough cards, so do a basic game
            round_cards.sort_unstable_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
            round_cards[0].0
        } else {
            let copied_decks = round_cards
                .iter()
                .zip(decks.iter())
                .map(|((player, card), deck)| {
                    assert_eq!(*player, deck.player);
                    Deck {
                        player: *player,
                        cards: deck.cards.iter().take(*card).cloned().collect(),
                    }
                })
                .collect::<Vec<_>>();
            let round_winner = play_the_game(copied_decks).player;
            // the only thing that doesn't quite work for 3+ players
            round_cards.sort_by(|(player, _), (_, _)| {
                if *player == round_winner {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
            round_winner
        };
        decks
            .iter_mut()
            .find(|deck| deck.player == round_winner)
            .unwrap()
            .add_to_bottom(round_cards);
        decks = decks.into_iter().filter(|deck| !deck.is_empty()).collect();
    }
    decks[0].clone()
}

fn do_the_thing(input: &str) -> IResult<&str, usize> {
    let (input, decks): (&str, Vec<Deck>) =
        separated_list1(pair(line_ending, line_ending), Deck::parse)(input)?;
    let winner = play_the_game(decks);
    Ok((input, winner.score()))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input).unwrap().1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second() {
        let input = "Player 1:
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
10";
        assert_eq!(do_the_thing(&input).unwrap().1, 291);
    }
}

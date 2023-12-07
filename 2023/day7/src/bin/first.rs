use std::fs;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard, // 5 unique
    OnePair, // 4 unique
    TwoPairs, // 3 unique
    ThreeOfAKind, // 3 unique
    FullHouse, // 2 unique
    FourOfAKind, // 2 unique
    FiveOfAKind, // 1 unique
}

#[derive(Debug, PartialEq, Eq)]
struct Cards([u16; 5]);

impl Cards {
    fn find_count(&self, count: u8) -> bool {
        let mut counts = [0; 13];
        for &card in self.0.iter() {
            counts[card.trailing_zeros() as usize] += 1;
        }
        counts.into_iter().any(|c| c == count)
    }
}

impl From<Vec<u16>> for Cards {
    fn from(cards: Vec<u16>) -> Self {
        Self(cards.try_into().unwrap())
    }
}

#[derive(Debug)]
struct Hand {
    cards: Cards,
    bid: i64,
    hand_type: HandType,
}

impl From<&Cards> for HandType {
    fn from(cards: &Cards) -> Self {
        match cards.0.iter().fold(0, |acc, &card| acc | card).count_ones() {
            5 => Self::HighCard,
            4 => Self::OnePair,
            3 => {
                // Either TwoPairs or ThreeOfAKind
                // TwoPairs is more common
                if cards.find_count(2) {
                    Self::TwoPairs
                } else {
                    Self::ThreeOfAKind
                }
            },
            2 => {
                // Either FullHouse or FourOfAKind
                // FullHouse is more common
                if cards.find_count(3) {
                    Self::FullHouse
                } else {
                    Self::FourOfAKind
                }
            },
            1 => Self::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.0.iter().zip(other.cards.0.iter()).find_map(|(a, b)| {
                match a.cmp(b) {
                    std::cmp::Ordering::Equal => None,
                    o => Some(o),
                }
            }).unwrap(),
            o => o,
        }
    }
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let cards: Cards = parts.next().unwrap().chars().map(|c| match c {
                'A' => 1 << 12,
                'K' => 1 << 11,
                'Q' => 1 << 10,
                'J' => 1 << 9,
                'T' => 1 << 8,
                _ => 1 << (c as u8 - b'2'),
            }).collect::<Vec<_>>().try_into().unwrap();
            let bid = parts.next().unwrap().parse().unwrap();
            let hand_type = (&cards).into();
            Hand { cards, bid, hand_type }
        })
        .collect()
}

pub fn run(input: &str) -> i64 {
    let mut hands = parse_hands(input);
    hands.sort_unstable();
    hands.into_iter().enumerate().fold(0, |acc, (i, hand)| {
        acc + hand.bid * (i as i64 + 1)
    })
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
" => 6440)]
    fn test(input: &str) -> i64 {
        run(input)
    }
}

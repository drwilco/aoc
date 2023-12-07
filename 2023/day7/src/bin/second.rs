use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    fs,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,     // 5 unique
    OnePair,      // 4 unique
    TwoPairs,     // 3 unique
    ThreeOfAKind, // 3 unique
    FullHouse,    // 2 unique
    FourOfAKind,  // 2 unique
    FiveOfAKind,  // 1 unique
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
struct Cards([u16; 5]);

impl Cards {
    fn find_count(&self, count: u8) -> bool {
        let mut counts = [0; 13];
        for &card in self.0.iter() {
            if card != 0 {
                counts[card.trailing_zeros() as usize] += 1;
            }
        }
        counts.into_iter().any(|c| c == count)
    }
}

impl From<Vec<u16>> for Cards {
    fn from(cards: Vec<u16>) -> Self {
        Self(cards.try_into().unwrap())
    }
}

impl Debug for Cards {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for &card in self.0.iter() {
            match card {
                0 => write!(f, "J")?,
                2048 => write!(f, "A")?,
                1024 => write!(f, "K")?,
                512 => write!(f, "Q")?,
                256 => write!(f, "T")?,
                128 => write!(f, "9")?,
                64 => write!(f, "8")?,
                32 => write!(f, "7")?,
                16 => write!(f, "6")?,
                8 => write!(f, "5")?,
                4 => write!(f, "4")?,
                2 => write!(f, "3")?,
                1 => write!(f, "2")?,
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

impl Display for Cards {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for &card in self.0.iter() {
            match card {
                0 => write!(f, "J")?,
                2048 => write!(f, "A")?,
                1024 => write!(f, "K")?,
                512 => write!(f, "Q")?,
                256 => write!(f, "T")?,
                128 => write!(f, "9")?,
                64 => write!(f, "8")?,
                32 => write!(f, "7")?,
                16 => write!(f, "6")?,
                8 => write!(f, "5")?,
                4 => write!(f, "4")?,
                2 => write!(f, "3")?,
                1 => write!(f, "2")?,
                _ => unreachable!(),
            }
        }
        Ok(())
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
        let jokers = (cards.0[0] == 0) as u8
            + (cards.0[1] == 0) as u8
            + (cards.0[2] == 0) as u8
            + (cards.0[3] == 0) as u8
            + (cards.0[4] == 0) as u8;
        let combined = cards.0[0] | cards.0[1] | cards.0[2] | cards.0[3] | cards.0[4];
        match combined.count_ones() {
            5 => Self::HighCard, // No jokers
            4 => Self::OnePair,  // 0 or 1 jokers: XYZAA or XYZAJ
            3 => {
                // 0 to 2 jokers: XYZJJ, XYZZJ, XYYZZ, or XYZZZ
                match jokers {
                    2 => Self::ThreeOfAKind, // XYZJJ so 3 of any
                    1 | 0 => {
                        // Either TwoPairs or ThreeOfAKind
                        // TwoPairs is more common, but with a joker we would
                        // search for 1, which would match XYZZJ which is 3 of a
                        // Kind. So search for the more desirable one
                        if cards.find_count(3 - jokers) {
                            Self::ThreeOfAKind
                        } else {
                            Self::TwoPairs
                        }
                    }
                    _ => unreachable!(),
                }
            }
            2 => {
                // We can have 0 to 3 jokers (because at least 2 non-jokers)
                match jokers {
                    2 | 3 => Self::FourOfAKind, // XYYJJ or XYJJJ, so 4 of Y
                    1 | 0 => {
                        // XXYYJ, XYYYJ, XXYYY, or XYYYY
                        // FourOfAKind or FullHouse
                        // FullHouse is more common
                        // With a joker we search for 2, which does not give false positives
                        if cards.find_count(3 - jokers) {
                            Self::FullHouse
                        } else {
                            Self::FourOfAKind
                        }
                    }
                    _ => unreachable!(),
                }
            }
            1 | 0 => Self::FiveOfAKind,
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
            Ordering::Equal => self.cards.cmp(&other.cards),
            o => o,
        }
    }
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let cards: Cards = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    'A' => 1 << 11,
                    'K' => 1 << 10,
                    'Q' => 1 << 9,
                    'T' => 1 << 8,
                    'J' => 0, // This way the jokers don't cause any ones in the bitfields
                    _ => 1 << (c as u8 - b'2'), // Shifted 0-7 times. We trust we covered all other cases above
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let bid = parts.next().unwrap().parse().unwrap();
            let hand_type = (&cards).into();
            Hand {
                cards,
                bid,
                hand_type,
            }
        })
        .collect()
}

pub fn run(input: &str) -> i64 {
    let mut hands = parse_hands(input);
    hands.sort_unstable();
    let result = hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i as i64 + 1));
    result
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
" => 5905)]
    fn test(input: &str) -> i64 {
        run(input)
    }
}

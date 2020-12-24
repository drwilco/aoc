use anyhow::Result;
use std::{iter::Flatten, vec};

#[derive(Clone, Debug)]
struct Ring<T> {
    items: Vec<T>,
}

impl<T: Clone + PartialEq> Ring<T> {
    fn position(&self, to_find: &T) -> usize {
        // panic if we can't find the requested item
        self.items
            .iter()
            .position(|item| *item == *to_find)
            .unwrap_or_else(|| panic!("Start item not found in ring"))
    }

    fn iter_from(&self, start: &T) -> Flatten<vec::IntoIter<&[T]>> {
        let position = self.position(start);
        let (last, first) = self.items.split_at(position);
        vec![first, last].into_iter().flatten()
    }

    fn take_after(&mut self, after: &T, amount: usize) -> Vec<T> {
        assert!(amount <= self.items.len());
        let position = (self.position(after) + 1) % self.items.len();
        if position + amount < self.items.len() {
            let mut take = self.items.split_off(position);
            let mut second = take.split_off(amount);
            self.items.append(&mut second);
            take
        } else {
            let mut take = self.items.split_off(position);
            take.append(&mut self.items);
            self.items = take.split_off(amount);
            take
        }
    }

    fn insert_after(&mut self, after: &T, mut to_insert: Vec<T>) {
        let position = (self.position(after) + 1) % self.items.len();
        if position == 0 {
            self.items.append(&mut to_insert);
        } else {
            let mut tail = self.items.split_off(position);
            self.items.append(&mut to_insert);
            self.items.append(&mut tail);
        }
    }
}

fn do_the_thing(input: &str, moves: usize) -> String {
    let mut ring = Ring {
        items: input.chars().map(|c| c.to_digit(10).unwrap()).collect(),
    };
    let mut current = ring.items[0];
    let highest = ring.items.iter().copied().max().unwrap();
    for _ in 0..moves {
        let lifted = ring.take_after(&current, 3);
        let mut destination = current;
        loop {
            destination = ((destination + highest - 2) % highest) + 1;
            if ring.items.contains(&destination) {
                break;
            }
        }
        ring.insert_after(&destination, lifted);
        current = ring.iter_from(&current).copied().nth(1).unwrap();
    }
    ring.iter_from(&1)
        .skip(1)
        .map(|n| n.to_string())
        .collect::<String>()
}

fn main() -> Result<()> {
    println!("{}", do_the_thing("872495136", 100));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("389125467", 10 => "92658374")]
    #[test_case("389125467", 100 => "67384529")]
    fn first(input: &str, moves: usize) -> String {
        do_the_thing(&input, moves)
    }

    #[test]
    fn ring_iter_from() {
        let items = vec![0, 1, 2, 8, 9, 10];

        let ring = Ring { items };
        assert_eq!(
            ring.iter_from(&0).copied().collect::<Vec<_>>(),
            vec![0, 1, 2, 8, 9, 10]
        );
        assert_eq!(
            ring.iter_from(&8).copied().collect::<Vec<_>>(),
            vec![8, 9, 10, 0, 1, 2]
        );
    }

    // from the middle
    #[test_case(0 => (vec![1, 2, 8], vec![0, 9, 10]))]
    #[test_case(1 => (vec![2, 8, 9], vec![0, 1, 10]))]
    // exactly the end
    #[test_case(2 => (vec![8, 9, 10], vec![0, 1, 2]))]
    // wrap around the end
    #[test_case(8 => (vec![9, 10, 0], vec![1, 2, 8]))]
    #[test_case(9 => (vec![10, 0, 1], vec![2, 8, 9]))]
    // exactly from the start
    #[test_case(10 => (vec![0, 1, 2], vec![8, 9, 10]))]
    fn ring_take(after: usize) -> (Vec<usize>, Vec<usize>) {
        let mut ring = Ring {
            items: vec![0, 1, 2, 8, 9, 10],
        };
        let take = ring.take_after(&after, 3);
        (take, ring.items)
    }

    #[test_case(0 => vec![0, 8, 9, 1, 2])]
    #[test_case(1 => vec![0, 1, 8, 9, 2])]
    #[test_case(2 => vec![0, 1, 2, 8, 9])]
    fn ring_insert(after: usize) -> Vec<usize> {
        let to_insert = vec![8, 9];
        let mut ring = Ring {
            items: vec![0, 1, 2],
        };
        ring.insert_after(&after, to_insert);
        ring.items
    }
}

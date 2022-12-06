use itertools::Itertools;
use std::{collections::VecDeque, fs};

fn do_the_thing(input: &str) -> usize {
    let mut buffer = VecDeque::<char>::new();
    input
        .chars()
        .enumerate()
        .find_map(|(index, character)| {
            buffer.push_back(character);
            if buffer.len() < 4 {
                return None;
            }
            if buffer.len() > 4 {
                buffer.pop_front();
            }
            // Clone, because we want to sort (for dedup), but not affect the original
            // as that would change which character gets removed next.
            let mut sorted_buffer = buffer.clone();
            sorted_buffer.make_contiguous().sort();
            if sorted_buffer.into_iter().dedup().count() == 4 {
                Some(index + 1)
            } else {
                None
            }
        })
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 7)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz" => 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg" => 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => 11)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

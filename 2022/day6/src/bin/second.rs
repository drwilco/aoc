use itertools::Itertools;
use std::fs;

fn do_the_thing(input: &str) -> usize {
    // Since with a VecDeque (see first.rs) we have to make_contiguous()
    // at every step, we might as well use a Vec and remove the first
    // element, which also causes a copy, but less copies than
    // make_contiguous().
    let mut buffer = Vec::<char>::new();
    input
        .chars()
        .enumerate()
        .find_map(|(index, character)| {
            if buffer.len() == 14 {
                buffer.remove(0);
            }
            buffer.push(character);
            if buffer.len() < 14 {
                return None;
            }
            // Clone, because we want to sort (for dedup), but not affect the original
            // as that would change which character gets removed next.
            let mut sorted_buffer = buffer.clone();
            sorted_buffer.sort();
            if sorted_buffer.into_iter().dedup().count() == 14 {
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

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz" => 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg" => 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => 26)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

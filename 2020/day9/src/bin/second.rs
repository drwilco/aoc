use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{collections::VecDeque, fs};

fn find_abberation(input: &Vec<u64>, preamble_length: usize) -> u64 {
    let mut ring = VecDeque::<u64>::new();
    input
        .iter()
        .enumerate()
        .find_map(|(index, n)| {
            if index < preamble_length {
                ring.push_back(*n);
                None
            } else {
                for (a, b) in ring.iter().tuple_combinations() {
                    if a + b == *n {
                        ring.push_back(*n);
                        ring.pop_front();
                        return None;
                    }
                }
                Some(*n)
            }
        })
        .unwrap()
}

fn find_sequence(input: &Vec<u64>, abberation: u64) -> Result<u64> {
    let input = &input[0..(input.iter().position(|&n| n == abberation).unwrap())];
    for length in 2..(input.len()) {
        for sequence in input.windows(length) {
            if sequence.iter().sum::<u64>() == abberation {
                return Ok(sequence.iter().min().unwrap() + sequence.iter().max().unwrap());
            }
        }
    }
    Err(anyhow!("Sequence not found"))
}

fn do_the_thing(input: &str, preamble_length: usize) -> Result<u64> {
    let input = input
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let abberation = find_abberation(&input, preamble_length);
    find_sequence(&input, abberation)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input, 25)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(62, do_the_thing(&input, 5).unwrap());
    }
}

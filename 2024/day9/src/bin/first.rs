#![feature(test)]

use itertools::Itertools;
use std::{fs, iter::repeat_n};

fn parse_input(input: &str) -> Vec<Option<usize>> {
    // Make pairs, so we can enumerate the pair for the file ID
    input
        .chars()
        .filter_map(|c| c.to_digit(10).and_then(|v| usize::try_from(v).ok()))
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(file_id, lengths)| {
            lengths.enumerate().flat_map(move |(index, length)| {
                if index == 0 {
                    repeat_n(Some(file_id), length)
                } else {
                    repeat_n(None, length)
                }
            })
        })
        .collect()
}

fn defrag(blocks: &mut [Option<usize>]) {
    assert!(!blocks.is_empty());
    let mut a = 0_usize;
    let mut b = blocks.len() - 1;
    while a < b {
        match (blocks[a].is_some(), blocks[b].is_some()) {
            (true, false) => {
                a += 1;
                b -= 1;
            }
            (true, true) => a += 1,
            (false, false) => b -= 1,
            (false, true) => {
                blocks.swap(a, b);
                a += 1;
                b -= 1;
            }
        }
    }
}

fn checksum(blocks: &[Option<usize>]) -> usize {
    blocks
        .iter()
        .enumerate()
        .map(|(position, block)| {
            block
                .map(|file_id| file_id * position)
                .unwrap_or(0)
        })
        .sum()
}

fn run(input: &str) -> usize {
    let mut blocks = parse_input(input);
    defrag(&mut blocks);
    checksum(&blocks)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    extern crate test as std_test;
    use super::*;
    use std_test::{black_box, Bencher};
    use test_case::test_case;

    #[bench]
    fn my_benchmark(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = black_box(&input);
        b.iter(|| run(input));
    }

    #[test_case("2333133121414131402" => 1928; "first example")]
    // 012345678
    // 022111222
    // 0*0 + 1*2 + 2*2 + 3*1 + 4*1 + 5*1 + 6*2 + 7*2 + 8*2 = 60
    #[test_case("12345" => 60; "second example")]
    fn test(input: &str) -> usize {
        run(input)
    }

    #[test_case("12345" => vec![Some(0), None, None, Some(1), Some(1), Some(1), None, None, None, None, Some(2), Some(2), Some(2), Some(2), Some(2)])]
    fn test_parse_input(input: &str) -> Vec<Option<usize>> {
        parse_input(input)
    }

    #[test_case("12345" => vec![Some(0), Some(2), Some(2), Some(1), Some(1), Some(1), Some(2), Some(2), Some(2), None, None, None, None, None, None])]
    fn test_defrag(input: &str) -> Vec<Option<usize>> {
        let mut blocks = parse_input(input);
        defrag(&mut blocks);
        blocks
    }
}

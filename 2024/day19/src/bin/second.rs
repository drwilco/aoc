#![feature(test)]

use std::{collections::HashMap, fs};

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect();
    let designs = designs.lines().collect();
    (towels, designs)
}

fn arrangements<'a>(
    towels: &[&str],
    design: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&count) = cache.get(design) {
        return count;
    }
    let count = towels
        .iter()
        .filter_map(|&towel| {
            if design.starts_with(towel) {
                Some(arrangements(towels, &design[towel.len()..], cache))
            } else {
                None
            }
        })
        .sum();
    cache.insert(design, count);
    count
}

fn solve(towels: &[&str], designs: &[&str]) -> usize {
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|&design| arrangements(towels, design, &mut cache))
        .sum()
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn run(input: &str) -> usize {
    let (towels, designs) = parse_input(input);
    solve(&towels, &designs)
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
    fn bench_parse(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = black_box(&input);
        b.iter(|| parse_input(input));
    }

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = parse_input(&input);
        let input = black_box(input);
        b.iter(|| solve(&input.0, &input.1));
    }

    #[test_case("r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
" => 16; "full example")]
    fn test(input: &str) -> usize {
        run(input)
    }

    const TOWELS: &[&str; 8] = &["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
    #[test_case(TOWELS, "brwrr" => 2; "example 1")]
    #[test_case(TOWELS, "bggr" => 1; "example 2")]
    #[test_case(TOWELS, "gbbr" => 4; "example 3")]
    #[test_case(TOWELS, "rrbgbr" => 6; "example 4")]
    #[test_case(TOWELS, "bwurrg" => 1; "example 5")]
    #[test_case(TOWELS, "brgr" => 2; "example 6")]
    fn test_single_design(towels: &[&str], design: &str) -> usize {
        arrangements(towels, design, &mut HashMap::new())
    }
}

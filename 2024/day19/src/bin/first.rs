#![feature(test)]

use std::fs;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect();
    let designs = designs.lines().collect();
    (towels, designs)
}

fn design_possible(towels: &[&str], design: &str) -> bool {
    for &towel in towels {
        if design.is_empty() {
            return true;
        }
        if design.starts_with(towel) {
            if design_possible(towels, &design[towel.len()..]) {
                return true;
            }
        }
    }
    false
}

fn solve(towels: &[&str], designs: &[&str]) -> usize {
    designs.iter().filter(|&&design| design_possible(towels, design)).count()
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
" => 6; "full example")]
    fn test(input: &str) -> usize {
        run(input)
    }

    const TOWELS: &[&str; 8] = &["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
    #[test_case(TOWELS, "brwrr" => true; "example 1")]
    #[test_case(TOWELS, "bggr" => true; "example 2")]
    #[test_case(TOWELS, "gbbr" => true; "example 3")]
    #[test_case(TOWELS, "rrbgbr" => true; "example 4")]
    #[test_case(TOWELS, "ubwu" => false; "example 5")]
    #[test_case(TOWELS, "bwurrg" => true; "example 6")]
    #[test_case(TOWELS, "brgr" => true; "example 7")]
    #[test_case(TOWELS, "bbrgwb" => false; "example 8")]
    fn test_single_design(towels: &[&str], design: &str) -> bool {
        design_possible(towels, design)
    }

}

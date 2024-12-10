#![feature(test)]

use std::fs;

fn parse_input(input: &str) -> usize {
    0
}

fn solve(input: usize) -> usize {
    input
}

#[must_use]
pub fn run(input: &str) -> usize {
    solve(parse_input(input))
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
        let input = parse_input(&fs::read_to_string("input.txt").unwrap());
        let input = black_box(input);
        b.iter(|| solve(input));
    }

    #[test_case("Hello world!" => 0)]
    fn test(input: &str) -> usize {
        run(input)
    }
}

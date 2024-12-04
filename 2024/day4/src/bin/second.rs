#![feature(test)]

use ndarray::Array;
use std::fs;

fn run(input: &str) -> usize {
    let (elements, (Some(width), height)) = input.lines().fold(
        (Vec::with_capacity(input.len()), (None, 0)),
        |(mut chars, (mut width, height)), line| {
            if width.is_none() {
                width = Some(line.len());
            }
            chars.extend(line.chars());
            (chars, (width, height + 1))
        },
    ) else {
        panic!("Empty input")
    };
    let grid = Array::from_shape_vec((height, width), elements).unwrap();
    grid.windows((3, 3))
        .into_iter()
        .filter(|window| {
            let center = window[[1, 1]];
            if center != 'A' {
                return false;
            }
            let corners = [
                window[[0, 0]],
                window[[0, 2]],
                window[[2, 0]],
                window[[2, 2]],
            ];
            corners == ['M', 'M', 'S', 'S']
                || corners == ['S', 'M', 'M', 'S']
                || corners == ['S', 'S', 'M', 'M']
                || corners == ['M', 'S', 'S', 'M']
        })
        .count()
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

    #[test_case("M.S
.A.
M.S
" => 1; "basic example")]
    #[test_case("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
" => 9; "example")]
    #[test_case(".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
" => 9; "example stripped")]
    #[test_case("MMM
MAS
SSS
" => 1; "only X, no +")]
    #[test_case("M.S
.A.
S.M
" => 0; "only MAS, no MAM and SAS")]
    fn test(input: &str) -> usize {
        run(input)
    }
}

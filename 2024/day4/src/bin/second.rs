#![feature(test)]
use itertools::Itertools;
use ndarray::Array;
use std::fs;

fn run(input: &str) -> usize {
    // Only looking for diagonals now
    let directions = [-1, 1]
        .into_iter()
        .cartesian_product([-1, 1])
        .collect::<Vec<_>>();
    let pattern = ['M', 'A', 'S'];
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
    grid.indexed_iter().fold(0, |acc, ((y, x), &c)| {
        // Let's scan for `A` because it's the center
        if c != pattern[1] {
            return acc;
        }
        let matches = directions
            .iter()
            // First find all the directions that we have a match in
            .filter(|&direction| {
                let mut matched = 0_usize;
                for (i, &p) in pattern.iter().enumerate() {
                    // We are centering on `A`, so start at -1, so we check -1,
                    // 0, 1
                    let i = isize::try_from(i).unwrap() - 1;
                    let (Some(y), Some(x)) = (
                        y.checked_add_signed(direction.0 * i),
                        x.checked_add_signed(direction.1 * i),
                    ) else {
                        break;
                    };
                    let Some(&to_check) = grid.get((y, x)) else {
                        break;
                    };
                    if to_check == p {
                        matched += 1;
                    } else {
                        break;
                    }
                }
                if matched == 3 {
                    true
                } else {
                    false
                }
            })
            // Since we're only checking diagonals, if we have 2 matches,
            // they're guaranteed to be perpendicular
            .count();
        if matches == 2 {
            acc + 1
        } else {
            acc
        }
    })
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    extern crate test as std_test;
    use super::*;
    use std_test::{Bencher, black_box};
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

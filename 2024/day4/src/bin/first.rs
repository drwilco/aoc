use itertools::Itertools;
use ndarray::Array;
use std::fs;

fn run(input: &str) -> usize {
    let directions = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|&(x, y)| !(x == 0 && y == 0))
        .collect::<Vec<_>>();
    let pattern = ['X', 'M', 'A', 'S'];
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
    grid.indexed_iter().fold(0, |mut acc, ((y, x), &c)| {
        if c != pattern[0] {
            return acc;
        }
        for direction in &directions {
            let mut matched = 0_usize;
            for (i, &p) in pattern[1..].iter().enumerate() {
                // We already checked for X, so we start from 1
                let factor = isize::try_from(i + 1).unwrap();
                let (Some(y), Some(x)) = (
                    y.checked_add_signed(direction.0 * factor),
                    x.checked_add_signed(direction.1 * factor),
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
                acc += 1;
            }
        }
        acc
    })
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("..X...
.SAMX.
.A..A.
XMAS.S
.X....
" => 4; "example1")]
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
" => 18; "example2")]
    #[test_case("....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
" => 18; "example3")]
    fn test(input: &str) -> usize {
        run(input)
    }
}

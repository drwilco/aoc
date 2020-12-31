use anyhow::{anyhow, Result};
use std::fs;

type TreeMap = Vec<Vec<bool>>;

fn parse_map(input: &str) -> Result<TreeMap> {
    input.lines().map(|line| {
        line.chars().map(|c| match c {
            '#' => Ok(true),
            '.' => Ok(false),
            _ => Err(anyhow!("invalid character")),
        }).collect()
    }).collect()
}

fn find_trees(input: &str, right: usize, down: usize) -> usize {
    let tree_map = parse_map(input)?;
    Ok(tree_map.into_iter().enumerate().filter_map(|(y, row)| {
        if y % down == 0 {
            let x = (y / down * right) % row.len();
                if row[x] {
                    return Some(())
                }
        }
        None
    }).count())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let angles = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result: usize = angles.into_iter().map(|(right, down)| find_trees(&input, right, down)).product();
    println!("{:?}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 1 => 2)]
    #[test_case(3, 1 => 7)]
    #[test_case(5, 1 => 3)]
    #[test_case(7, 1 => 4)]
    #[test_case(1, 2 => 2)]
    fn second(right: usize, down: usize) -> usize {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        find_trees(input, right, down).unwrap()
    }
}

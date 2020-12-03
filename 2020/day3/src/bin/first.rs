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

fn find_trees(input: &str, angle: usize) -> Result<usize> {
    let tree_map = parse_map(input)?;
    Ok(tree_map.into_iter().enumerate().filter_map(|(y, row)| {
        let x = (y * angle) % row.len();
        if row[x] {
            Some(())
        } else {
            None
        }
    }).count())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", find_trees(&input, 3)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
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
        assert_eq!(find_trees(input, 3).unwrap(), 7);
    }
}

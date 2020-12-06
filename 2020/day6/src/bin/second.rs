use anyhow::{Result};
use std::{collections::HashSet, fs};
use std::ops::BitAnd;

fn get_count(input: &str) -> usize {
    input.split("\n\n").map(|group| {
        group.lines().map(|line| {
            line.chars().collect::<HashSet<char>>()
        }).fold(None, |acc: Option<HashSet<char>>, set| {
            if let Some(acc) = acc {
                Some(acc.bitand(&set))
            } else {
                Some(set)
            }
        }).unwrap().len()
    }).sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", get_count(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abc" => 3)]
#[test_case("a
b
c" => 0)]
#[test_case("ab
ac" => 1)]
#[test_case("a
a
a
a" => 1)]
#[test_case("b" => 1)]
    fn id(input: &str) -> usize {
        get_count(input)
    }

    #[test]
    fn second() {
        assert_eq!(get_count("abc

a
b
c

ab
ac

a
a
a
a

b"), 6);
    }
}
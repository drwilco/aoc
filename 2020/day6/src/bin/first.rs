use anyhow::{Result};
use std::{collections::HashSet, fs};

fn get_count(input: &str) -> usize {
    input.split("\n\n").map(|group| {
        let mut set = HashSet::<char>::new();
        for line in group.lines() {
            for answer in line.chars() {
                set.insert(answer);
            }
        }
        set.len()
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
c" => 3)]
#[test_case("ab
ac" => 3)]
#[test_case("a
a
a
a" => 1)]
#[test_case("b" => 1)]
    fn id(input: &str) -> usize {
        get_count(input)
    }

    #[test]
    fn first() {
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

b"), 11);
    }
}
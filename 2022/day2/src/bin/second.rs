#![allow(clippy::identity_op)]

use std::fs;
use std::io::Result;

fn do_the_thing(input: &str) -> u64 {
    input
        .lines()
        .into_iter()
        .map(|line| {
            match line {
                // A = Rock
                // B = Paper
                // C = Scissors
                // X = Lose (0 point)
                // Y = Draw (3 points)
                // Z = Win (6 points)
                // Rock = 1, Paper = 2, Scissors = 3
                "A X" => 3 + 0,
                "A Y" => 1 + 3,
                "A Z" => 2 + 6,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 2 + 0,
                "C Y" => 3 + 3,
                "C Z" => 1 + 6,
                _ => panic!("invalid input")
            }
        })
        .sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("A Y
B X
C Z
" => 12)]
    fn first(input: &str) -> u64 {
        do_the_thing(&input)
    }
}

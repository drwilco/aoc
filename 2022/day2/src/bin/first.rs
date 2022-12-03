#![allow(clippy::identity_op)]

use std::fs;

fn do_the_thing(input: &str) -> u64 {
    input
        .lines()
        .into_iter()
        .map(|line| {
            match line {
                // A = Rock
                // B = Paper
                // C = Scissors
                // X = Rock (1 point)
                // Y = Paper (2 points)
                // Z = Scissors (3 points)
                // Win = 6, draw = 3, loss = 0
                "A X" => 1 + 3,
                "A Y" => 2 + 6,
                "A Z" => 3 + 0,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 1 + 6,
                "C Y" => 2 + 0,
                "C Z" => 3 + 3,
                _ => panic!("invalid input")
            }
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("A Y
B X
C Z
" => 15)]
    fn first(input: &str) -> u64 {
        do_the_thing(&input)
    }
}

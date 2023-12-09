use std::fs;

use rayon::prelude::*;

fn differences(sequence: &[i64]) -> Vec<i64> {
    sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

pub fn run(input: &str) -> i64 {
    let sequences = input.lines().par_bridge().map(|line| {
        line.split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });
    sequences.map(|sequence| {
        let mut collection = Vec::new();
        let mut diffs = differences(&sequence);
        collection.push(sequence);
        while !diffs.iter().all(|&diff| diff == 0) {
            let new_diffs = differences(&diffs);
            collection.push(diffs);
            diffs = new_diffs;
        }
        let result = collection.into_iter().rev().fold(0, |acc, diffs| {
            acc + diffs.last().unwrap()
        });
        result
    }).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
" => 114)]
    fn test(input: &str) -> i64 {
        run(&input)
    }
}

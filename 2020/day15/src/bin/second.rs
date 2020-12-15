use anyhow::Result;
use std::{collections::HashMap, fs};

struct Day15 {
    starting: Vec<usize>,
    numbers: HashMap<usize, usize>,
    last: Option<usize>,
    index: usize,
}

impl Iterator for Day15 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.index < self.starting.len() {
            self.starting[self.index]
        } else {
            match self.numbers.get(&self.last.unwrap()) {
                None => 0,
                Some(seen_at) => (self.index - 1) - *seen_at,
            }
        };
        if let Some(last) = self.last {
            self.numbers.insert(last, self.index - 1);
        }
        self.last = Some(result);
        self.index += 1;
        Some(result)
    }
}

impl From<Vec<usize>> for Day15 {
    fn from(starting: Vec<usize>) -> Self {
        Day15 {
            last: None,
            numbers: HashMap::new(),
            starting: starting,
            index: 0,
        }
    }
}

fn do_the_thing(input: &str, nth: usize) -> usize {
    let mut day15: Day15 = input
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>()
        .into();

    day15.nth(nth - 1).unwrap()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input, 30000000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0,3,6", 2020 => 436)]
    #[test_case("1,3,2", 2020 => 1)]
    #[test_case("2,1,3", 2020 => 10)]
    #[test_case("1,2,3", 2020 => 27)]
    #[test_case("2,3,1", 2020 => 78)]
    #[test_case("3,2,1", 2020 => 438)]
    #[test_case("3,1,2", 2020 => 1836)]
    #[test_case("0,3,6", 30000000 => 175594)]
    #[test_case("1,3,2", 30000000 => 2578)]
    #[test_case("2,1,3", 30000000 => 3544142)]
    #[test_case("1,2,3", 30000000 => 261214)]
    #[test_case("2,3,1", 30000000 => 6895259)]
    #[test_case("3,2,1", 30000000 => 18)]
    #[test_case("3,1,2", 30000000 => 362)]
    fn second(input: &str, nth: usize) -> usize {
        do_the_thing(&input, nth)
    }

    #[test]
    fn test_iterator() {
        let iter: Day15 = vec![0, 3, 6].into();
        assert_eq!(
            iter.take(10).collect::<Vec<_>>(),
            vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0]
        );
    }
}

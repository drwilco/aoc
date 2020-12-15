use anyhow::Result;
use std::fs;

struct Day15 {
    numbers: Vec<usize>,
    position: usize,
}

impl Iterator for Day15 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position;
        self.position += 1;
        if position < self.numbers.len() {
            Some(self.numbers[position])
        } else {
            assert_eq!(position, self.numbers.len());
            let last = *self.numbers.last().unwrap();
            let new = match self.numbers.iter().rev().skip(1).position(|&n| n == last) {
                None => 0,
                Some(n) => n + 1,
            };
            self.numbers.push(new);
            Some(new)
        }
    }
}

impl From<Vec<usize>> for Day15 {
    fn from(numbers: Vec<usize>) -> Self {
        Day15 {
            numbers,
            position: 0,
        }
    }
}

fn do_the_thing(input: &str) -> usize {
    let mut day15: Day15 = input
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>()
        .into();

    day15.nth(2019).unwrap()
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

    #[test_case("0,3,6" => 436)]
    #[test_case("1,3,2" => 1)]
    #[test_case("2,1,3" => 10)]
    #[test_case("1,2,3" => 27)]
    #[test_case("2,3,1" => 78)]
    #[test_case("3,2,1" => 438)]
    #[test_case("3,1,2" => 1836)]
    fn first(input: &str) -> usize {
        do_the_thing(&input)
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

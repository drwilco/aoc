use std::fs;

fn do_the_thing(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|line| {
                    line.as_bytes()
                        .iter()
                        .map(|c| {
                            1 << match c {
                                65..=90 => c - 38,  // A -> 27, Z -> 52
                                97..=122 => c - 96, // a -> 1, z -> 26
                                _ => panic!("invalid input"),
                            }
                        })
                        .fold(0, |acc, val| acc | val)
                })
                .fold(u64::MAX, |acc: u64, val| acc & val)
                .trailing_zeros() as u64
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

    #[test_case("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
" => 70)]
    fn second(input: &str) -> u64 {
        do_the_thing(&input)
    }
}

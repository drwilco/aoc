use std::fs;

fn do_the_thing(input: &str) -> u64 {
    input.lines()
        .map(|line| {
            let half_length = line.len() / 2;
            let bitmasks = line
                .as_bytes()
                .chunks(half_length)
                .map(|chars| {
                    chars
                        .iter()
                        .map(|c| {
                            1 << match c {
                                65..=90 => c - 38, // A -> 27, Z -> 52
                                97..=122 => c - 96, // a -> 1, z -> 26
                                _ => panic!("invalid input")
                            }
                            })
                        .fold(0, |acc, v| acc | v) 
                }).collect::<Vec<u64>>();
            let in_common = bitmasks[0] & bitmasks[1];
            assert!(in_common.count_ones() == 1);
            in_common.trailing_zeros() as u64
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
" => 157)]
    fn first(input: &str) -> u64 {
        do_the_thing(&input)
    }
}

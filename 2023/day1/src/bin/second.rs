use lazy_static::lazy_static;
use std::{fs, collections::HashMap};

lazy_static! {
    static ref DIGITS: HashMap<&'static str, usize> = 
    [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ].into_iter().collect();
}

fn do_the_thing(input: &str) -> usize {
    input.lines().map(|line| {
        let mut line_f = line.to_string();
        let mut line_l = line_f.clone();
        let mut first = None;
        let mut last = None;
        'first_outer: while !line_f.is_empty() {
            for digit in DIGITS.keys() {
                if line_f.starts_with(digit) {
                    first = Some(DIGITS.get(digit).unwrap());
                    break 'first_outer;
                }
            }
            line_f.remove(0);
        };
        'last_outer: while !line_l.is_empty() {
            for digit in DIGITS.keys() {
                if line_l.ends_with(digit) {
                    last = Some(DIGITS.get(digit).unwrap());
                    break 'last_outer;
                }
            }
            line_l.pop();
        };
        println!("{}{}", first.unwrap(), last.unwrap());
        first.unwrap() * 10 + last.unwrap()  
    }).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen" => 281)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

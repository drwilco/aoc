use std::{cmp::Ordering, fs};

use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum Value {
    Number(u64),
    List(List),
}

// NewType pattern to implement Ord
#[derive(Debug)]
struct List(Vec<Value>);

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut iter = self.0.iter().zip(other.0.iter());
        loop {
            match iter.next() {
                Some((a, b)) => match a.cmp(b) {
                    Ordering::Equal => continue,
                    other => return other,
                },
                None => return self.0.len().cmp(&other.0.len()),
            }
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for List {}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => a.cmp(b),
            (Value::Number(a), Value::List(b)) => List(vec![Value::Number(*a)]).cmp(b),
            (Value::List(a), Value::Number(b)) => a.cmp(&List(vec![Value::Number(*b)])),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

// NewType pattern so that Rust Analyzer displays the type
// as Packet instead of List
#[derive(PartialEq, PartialOrd)]
struct Packet(List);

type Pair = (Packet, Packet);

fn parse_u64(input: &str) -> IResult<&str, u64> {
    let (input, number) = digit1(input)?;
    Ok((input, number.parse().unwrap()))
}

fn parse_list(input: &str) -> IResult<&str, List> {
    let (input, _) = char('[')(input)?;
    let (input, values) = separated_list0(char(','), parse_value)(input)?;
    let (input, _) = char(']')(input)?;
    Ok((input, List(values)))
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((map(parse_u64, Value::Number), map(parse_list, Value::List)))(input)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    map(terminated(parse_list, line_ending), Packet)(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    tuple((parse_packet, parse_packet))(input)
}

fn do_the_thing(input: &str) -> usize {
    let (input, pairs) = separated_list1(line_ending, parse_pair)(input).unwrap();
    assert!(input.is_empty());

    pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, (a, b))| if a < b { Some(i + 1) } else { None })
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

    #[test_case("[1,1,3,1,1]
[1,1,5,1,1]
" => 1)]
    #[test_case("[[1],[2,3,4]]
[[1],4]
" => 1)]
    #[test_case("[9]
[[8,7,6]]
"  => 0)]
    #[test_case("[[4,4],4,4]
[[4,4],4,4,4]
" => 1)]
    #[test_case("[7,7,7,7]
[7,7,7]
" => 0)]
    #[test_case("[]
[3]
" => 1)]
    #[test_case("[[[]]]
[[]]
" => 0)]
    #[test_case("[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
" => 13)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

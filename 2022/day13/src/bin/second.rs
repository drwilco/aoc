use std::{cmp::Ordering, fs};

use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending},
    combinator::{map, opt},
    multi::{many1, separated_list0},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Clone, Debug)]
enum Value {
    Number(u64),
    List(List),
}

#[derive(Clone, Debug)]
struct List(Vec<Value>);

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut iter = self.0.iter().zip(other.0.iter());
        while let Some((a, b)) = iter.next() {
            match a.cmp(b) {
                Ordering::Equal => continue,
                other => return other,
            };
        }
        self.0.len().cmp(&other.0.len())
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
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Packet(List);

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
    map(
        terminated(parse_list, tuple((line_ending, opt(line_ending)))),
        Packet,
    )(input)
}

const DIVIDER_PACKETS: &str = "[[2]]
[[6]]
";

fn do_the_thing(input: &str) -> usize {
    let (input, mut packets) = many1(parse_packet)(input).unwrap();
    assert!(input.is_empty());
    let (input, divider_packets) = many1(parse_packet)(DIVIDER_PACKETS).unwrap();
    assert!(input.is_empty());
    packets.append(&mut divider_packets.clone());
    packets.sort_unstable();
    let mut indeces = packets
        .into_iter()
        .enumerate()
        .filter_map(|(index, packet)| {
            divider_packets
                .iter()
                .find(|&p| p == &packet)
                .map(|_| index + 1)
        });
    // could have done .product() above, but this doesn't make the iterator
    // go through all the elements past the second divider packet
    indeces.next().unwrap() * indeces.next().unwrap()
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
" => 140)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

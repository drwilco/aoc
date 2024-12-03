use anyhow::Result;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use std::{fs, str::FromStr};

fn parse_num<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    map(digit1, |digit_str: &str| digit_str.parse::<T>().unwrap())(input)
}

#[derive(Debug)]
struct Rule<'a> {
    _name: &'a str,
    low_min: usize,
    low_max: usize,
    high_min: usize,
    high_max: usize,
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, name) = is_not(":")(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, low_min) = parse_num(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, low_max) = parse_num(input)?;
    let (input, _) = tag(" or ")(input)?;
    let (input, high_min) = parse_num(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, high_max) = parse_num(input)?;
    Ok((
        input,
        Rule {
            _name: name,
            low_min,
            low_max,
            high_min,
            high_max,
        },
    ))
}

type Ticket = Vec<usize>;

fn parse_input(input: &str) -> IResult<&str, (Vec<Rule>, Ticket, Vec<Ticket>)> {
    let mut parse_ticket = separated_list1(tag(","), parse_num);

    let (input, rules) = separated_list1(line_ending, parse_rule)(input)?;
    let (input, _) = tuple((many1(line_ending), tag("your ticket:"), line_ending))(input)?;
    let (input, my_ticket) = parse_ticket(input)?;
    let (input, _) = tuple((many1(line_ending), tag("nearby tickets:"), line_ending))(input)?;
    let (input, nearby_tickets) = separated_list1(line_ending, parse_ticket)(input)?;
    Ok((input, (rules, my_ticket, nearby_tickets)))
}

fn do_the_thing(input: &str) -> usize {
    let (_, (rules, _, nearby_tickets)) = parse_input(input).unwrap();
    nearby_tickets
        .into_iter()
        .flat_map(|ticket| {
            ticket
                .into_iter()
                .filter(|&field| {
                    rules
                        .iter()
                        .filter(|rule| {
                            (rule.low_min..=rule.low_max).contains(&field)
                                || (rule.high_min..=rule.high_max).contains(&field)
                        })
                        .count()
                        == 0
                })
                .collect::<Vec<_>>()
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

    #[test_case("class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12" => 71)]
    fn first(input: &str) -> usize {
        do_the_thing(&input)
    }
}

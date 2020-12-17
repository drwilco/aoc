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
    name: &'a str,
    low_min: usize,
    low_max: usize,
    high_min: usize,
    high_max: usize,
    position: Option<usize>,
}

impl Rule<'_> {
    fn matches(&self, input: &usize) -> bool {
        (self.low_min..=self.low_max).contains(input)
            || (self.high_min..=self.high_max).contains(input)
    }
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
            name,
            low_min,
            low_max,
            high_min,
            high_max,
            position: None,
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

fn is_valid_ticket(ticket: &[usize], rules: &[Rule]) -> bool {
    ticket
        .iter()
        .filter(|&field| rules.iter().filter(|rule| rule.matches(field)).count() == 0)
        .count()
        == 0
}

fn do_the_thing(input: &str, field_prefix: &str) -> usize {
    let (_, (mut rules, my_ticket, nearby_tickets)) = parse_input(input).unwrap();
    let num_fields = my_ticket.len();
    let valid_tickets = nearby_tickets
        .into_iter()
        .filter(|ticket| is_valid_ticket(ticket, &rules))
        .collect::<Vec<_>>();
    let mut todo_fields = (0..num_fields).into_iter().collect::<Vec<usize>>();
    while rules.iter().filter(|rule| rule.position.is_none()).count() > 0 {
        todo_fields = todo_fields
            .into_iter()
            .filter(|position| {
                let column = valid_tickets
                    .iter()
                    .map(|ticket| ticket[*position])
                    .collect::<Vec<_>>();
                let mut unmapped_valid_rules = rules
                    .iter_mut()
                    .filter(|rule| match rule.position {
                        Some(_) => false, // if this rule has been mapped, we can skip it
                        None => column.iter().find(|field| !rule.matches(*field)).is_none(),
                    })
                    .collect::<Vec<_>>();
                if unmapped_valid_rules.len() == 1 {
                    unmapped_valid_rules[0].position = Some(*position);
                    false
                } else {
                    true
                }
            })
            .collect();
    }
    rules
        .into_iter()
        .filter_map(|rule| {
            if rule.name.starts_with(field_prefix) {
                Some(my_ticket[rule.position.unwrap()])
            } else {
                None
            }
        })
        .product()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input, "departure"));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("class" => 12)]
    #[test_case("row" => 11)]
    #[test_case("seat" => 13)]
    fn second(prefix: &str) -> usize {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        do_the_thing(input, prefix)
    }
}

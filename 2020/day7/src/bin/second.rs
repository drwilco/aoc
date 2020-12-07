use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space1},
    combinator::{map, opt, value},
    multi::separated_list1,
    sequence::pair,
    IResult,
};
use std::{collections::HashMap, fs};

type Rule<'a> = (&'a str, Vec<(usize, &'a str)>);
type RuleList<'a> = HashMap<&'a str, Vec<(usize, &'a str)>>;

fn parse_num(input: &str) -> IResult<&str, usize> {
    map(digit1, |digit_str: &str| {
        digit_str.parse::<usize>().unwrap()
    })(input)
}

fn parse_colored_bag(input: &str) -> IResult<&str, &str> {
    let (input, result) = take_until(" bag")(input)?;
    let (input, _) = tag(" bag")(input)?;
    let (input, _) = opt(tag("s"))(input)?;
    Ok((input, result))
}

fn parse_count_and_color(input: &str) -> IResult<&str, (usize, &str)> {
    let (input, count) = parse_num(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = parse_colored_bag(input)?;
    Ok((input, (count, color)))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, outer) = parse_colored_bag(input)?;
    let (input, _) = tag(" contain ")(input)?;
    let (input, inner) = alt((
        value(vec![], tag("no other bags")),
        separated_list1(pair(tag(","), space1), parse_count_and_color),
    ))(input)?;
    let (input, _) = tag(".")(input)?;
    Ok((input, (outer, inner)))
}

fn parse_rule_list(input: &str) -> IResult<&str, RuleList> {
    let (input, rules) = separated_list1(tag("\n"), parse_rule)(input)?;
    Ok((input, rules.into_iter().collect()))
}

fn find_number_of_inner_bags(wanted: &str, rules: &RuleList) -> usize {
    let contents = &rules[wanted];
    if contents.len() > 0 {
        contents
            .iter()
            .map(|(count, color)| count * (1 + find_number_of_inner_bags(color, &rules)))
            .sum()
    } else {
        0
    }
}

fn find_total_number_of_inner_bags(wanted: &str, input: &str) -> usize {
    let (_, rules) = parse_rule_list(&input).unwrap();
    find_number_of_inner_bags(wanted, &rules)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!(
        "{:?}",
        find_total_number_of_inner_bags("shiny gold", &input)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("light red bags contain 1 bright white bag, 2 muted yellow bags." => ("light red", vec![(1, "bright white"), (2, "muted yellow")]))]
    #[test_case("faded blue bags contain no other bags." => ("faded blue", vec![]))]
    fn rule(input: &str) -> (&str, Vec<(usize, &str)>) {
        let (_input, rule) = parse_rule(input).unwrap();
        rule
    }

    #[test_case("shiny gold" => 32; "full example")]
    #[test_case("vibrant plum" => 11)]
    #[test_case("dark olive" => 7)]
    fn second(wanted: &str) -> usize {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        find_total_number_of_inner_bags(wanted, input)
    }
}

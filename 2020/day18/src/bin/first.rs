use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, value},
    multi::{many1, separated_list1},
    IResult,
};
use std::fs;

fn parse_number(input: &str) -> IResult<&str, Component> {
    let (input, number) = map(digit1, |digit_str: &str| {
        digit_str.parse::<usize>().unwrap()
    })(input)?;
    Ok((input, Component::Number(number)))
}

fn parse_sub_expression(input: &str) -> IResult<&str, Component> {
    let (input, _) = tag("(")(input)?;
    let (input, expression) = parse_expression(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Component::Expression(expression)))
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, components) = many1(alt((
        value(Component::Add, tag("+")),
        value(Component::Multiply, tag("*")),
        parse_number,
        parse_sub_expression,
    )))(input)?;
    Ok((input, Expression { components }))
}

#[derive(Clone, Debug)]
enum Component {
    Add,
    Multiply,
    Number(usize),
    Expression(Expression),
}

#[derive(Clone, Debug)]
struct Expression {
    components: Vec<Component>,
}

impl Expression {
    fn result(&self) -> usize {
        let (result, _) =
            self.components
                .iter()
                .fold((0, None), |(mut acc, previous), component| {
                    match component {
                        Component::Number(_) | Component::Expression(_) => {
                            let value = match component {
                                Component::Number(number) => *number,
                                Component::Expression(expression) => expression.result(),
                                _ => unreachable!(),
                            };
                            match previous {
                                None => acc = value,
                                Some(&Component::Add) => acc += value,
                                Some(&Component::Multiply) => acc *= value,
                                Some(_) => panic!("invalid order of components"),
                            };
                        }
                        Component::Add | Component::Multiply => (),
                    };
                    (acc, Some(component))
                });
        result
    }
}

fn do_the_thing(input: &str) -> usize {
    let input = &input.chars().filter(|c| *c != ' ').collect::<String>();
    let (_, expressions) = separated_list1(line_ending, parse_expression)(input).unwrap();
    expressions
        .into_iter()
        .map(|expression| expression.result())
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

    #[test_case("1 + 2 * 3 + 4 * 5 + 6" => 71)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))" => 51)]
    #[test_case("2 * 3 + (4 * 5)" => 26)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)" => 437)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))" => 12240)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2" => 13632)]
    #[test_case("2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)" => 463)]
    fn first(input: &str) -> usize {
        do_the_thing(&input)
    }
}

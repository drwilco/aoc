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
    fn result(self) -> usize {
        // highest precedence
        let components = self
            .components
            .into_iter()
            .map(|component| {
                if let Component::Expression(expression) = component {
                    Component::Number(expression.result())
                } else {
                    component
                }
            })
            .collect::<Vec<_>>();
        // next precedence is addition
        let components = components
            .into_iter()
            .fold(Vec::new(), |mut acc, component| {
                if acc.len() == 0 {
                    acc.push(component);
                    return acc;
                }
                match component {
                    Component::Number(value) => {
                        // if we're looking at a number we know it's not the first (if above) and we should
                        // have at least another number and operator. If that's a multiply, just throw the
                        // current value on the stack. But if it's an add, replace both with result of addition
                        match acc.last().unwrap() {
                            Component::Add => {
                                let _ = acc.pop().unwrap();
                                if let Component::Number(previous_value) = acc.pop().unwrap() {
                                    acc.push(Component::Number(value + previous_value));
                                } else {
                                    panic!("components in unexpected order");
                                }
                            }
                            Component::Multiply => acc.push(component),
                            _ => unreachable!(),
                        }
                    }
                    _ => acc.push(component),
                }
                acc
            });
        // now we should just have numbers and multiplies alternated
        components
            .into_iter()
            .step_by(2)
            .map(|component| {
                if let Component::Number(value) = component {
                    value
                } else {
                    unreachable!()
                }
            })
            .product()
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

    #[test_case("1 + 2 * 3 + 4 * 5 + 6" => 231)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))" => 51)]
    #[test_case("2 * 3 + (4 * 5)" => 46)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)" => 1445)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))" => 669060)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2" => 23340)]
    #[test_case("2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)" => 1491)]
    fn second(input: &str) -> usize {
        do_the_thing(&input)
    }
}

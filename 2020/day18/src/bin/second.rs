use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::separated_list1,
    IResult,
};
use std::fs;

// an expression is basically a multiplication of additions,
// and an addition is a bunch of numbers or sub-expressions.
// And since a sub-expression is basically an expression with
// parentheses around them, and an expression is a multiplication.
#[derive(Clone, Debug)]
enum Component {
    Number(usize),
    Expression(Multiplication),
}
#[derive(Clone, Debug)]
struct Multiplication(Vec<Addition>);
#[derive(Clone, Debug)]
struct Addition(Vec<Component>);

fn parse_number(input: &str) -> IResult<&str, Component> {
    let (input, number) = map(digit1, |digit_str: &str| {
        digit_str.parse::<usize>().unwrap()
    })(input)?;
    Ok((input, Component::Number(number)))
}

fn parse_sub_expression(input: &str) -> IResult<&str, Component> {
    let (input, _) = tag("(")(input)?;
    let (input, expression) = parse_multiplication(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Component::Expression(expression)))
}

fn parse_multiplication(input: &str) -> IResult<&str, Multiplication> {
    let (input, additions) = separated_list1(tag("*"), parse_addition)(input)?;
    Ok((input, Multiplication(additions)))
}

fn parse_addition(input: &str) -> IResult<&str, Addition> {
    let (input, components) =
        separated_list1(tag("+"), alt((parse_number, parse_sub_expression)))(input)?;
    Ok((input, Addition(components)))
}

impl Addition {
    fn result(self) -> usize {
        self.0
            .into_iter()
            .map(|component| match component {
                Component::Number(value) => value,
                Component::Expression(multiplication) => multiplication.result(),
            })
            .sum()
    }
}
impl Multiplication {
    fn result(self) -> usize {
        self.0
            .into_iter()
            .map(|addition| addition.result())
            .product()
    }
}

fn do_the_thing(input: &str) -> usize {
    let input = &input.chars().filter(|c| *c != ' ').collect::<String>();
    let (_, expressions) = separated_list1(line_ending, parse_multiplication)(input).unwrap();
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

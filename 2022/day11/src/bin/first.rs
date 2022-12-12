use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    MultiplyBySelf,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisible_by: usize,
    true_target: usize,
    false_target: usize,
    actions: usize,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, number) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    Ok((input, number))
}

fn parse_items(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), parse_usize)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, items))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("  Operation: new = old ")(input)?;
    let (input, operation) = terminated(
        alt((
            map(preceded(tag("+ "), parse_usize), Operation::Add),
            map(tag("* old"), |_| Operation::MultiplyBySelf),
            map(preceded(tag("* "), parse_usize), Operation::Multiply),
        )),
        line_ending,
    )(input)?;
    Ok((input, operation))
}

fn parse_divisible_by(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("  Test: divisible by ")(input)?;
    let (input, divisible_by) = parse_usize(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, divisible_by))
}

fn parse_true_false(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, _) = tag("    If true: throw to monkey ")(input)?;
    let (input, true_target) = parse_usize(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = tag("    If false: throw to monkey ")(input)?;
    let (input, false_target) = parse_usize(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, (true_target, false_target)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, _) = parse_usize(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = line_ending(input)?;
    let (input, items) = parse_items(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, divisible_by) = parse_divisible_by(input)?;
    let (input, (true_target, false_target)) = parse_true_false(input)?;
    Ok((
        input,
        Monkey {
            items,
            operation,
            divisible_by,
            true_target,
            false_target,
            actions: 0,
        },
    ))
}

fn do_the_thing(input: &str) -> usize {
    let (input, mut monkeys) = separated_list1(line_ending, parse_monkey)(input).unwrap();
    assert!(input.is_empty());
    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let mut items = Vec::new();
            for item in &monkeys[index].items {
                let mut new_item = match &monkeys[index].operation {
                    Operation::Add(i) => *item + *i,
                    Operation::Multiply(i) => *item * *i,
                    Operation::MultiplyBySelf => *item * *item,
                };
                new_item /= 3;
                items.push(new_item);
            }
            monkeys[index].actions += items.len();
            for item in items {
                let target = if item % monkeys[index].divisible_by == 0 {
                    monkeys[index].true_target
                } else {
                    monkeys[index].false_target
                };
                monkeys[target].items.push(item);
            }
            monkeys[index].items.clear();
        }
    }
    let mut scores = monkeys.into_iter().map(|m| m.actions).collect::<Vec<_>>();
    scores.sort_unstable();
    scores.reverse();
    scores[0] * scores[1]
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
" => 10605)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

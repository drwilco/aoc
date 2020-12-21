use anyhow::Result;
use fs::File;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{pair, terminated},
    IResult,
};
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
    path::Path,
    str::FromStr,
};

fn parse_num<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    map(digit1, |digit_str: &str| digit_str.parse::<T>().unwrap())(input)
}

fn parse_literal(input: &str) -> IResult<&str, Component> {
    let (input, _) = tag("\"")(input)?;
    let (input, literal) = anychar(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, Component::Literal(literal)))
}

fn parse_reference(input: &str) -> IResult<&str, Component> {
    let (input, reference) = parse_num(input)?;
    Ok((input, Component::Reference(reference)))
}

fn parse_rule(input: &str) -> IResult<&str, (usize, Rule)> {
    pair(
        terminated(parse_num, tag(": ")),
        separated_list1(
            tag(" | "),
            separated_list1(tag(" "), alt((parse_reference, parse_literal))),
        ),
    )(input)
}

type Branch = Vec<Component>;
type Rule = Vec<Branch>;

#[derive(Debug)]
enum Component {
    Literal(char),
    Reference(usize),
}

fn parser_for_component(
    output: &mut File,
    rules: &HashMap<usize, Rule>,
    component: &Component,
) -> io::Result<()> {
    match component {
        Component::Reference(r) => parser_for_rule(output, rules, *r),
        Component::Literal(c) => output.write_all(format!("char('{}')", c).as_bytes()),
    }
}

fn parser_for_branch(
    output: &mut File,
    rules: &HashMap<usize, Rule>,
    branch: &Branch,
) -> io::Result<()> {
    // either 1 or 2 elements
    if branch.len() == 2 {
        output.write_all(b"recognize(pair(")?;
    }
    parser_for_component(output, rules, &branch[0])?;
    if branch.len() == 2 {
        output.write_all(b",")?;
        parser_for_component(output, rules, &branch[1])?;
        output.write_all(b"))")?;
    }
    Ok(())
}

fn parser_for_rule(
    output: &mut File,
    rules: &HashMap<usize, Rule>,
    rule_number: usize,
) -> io::Result<()> {
    let rule = &rules[&rule_number];
    // either 1 or 2 branches
    if rule.len() == 2 {
        output.write_all(b"alt((")?;
    }
    parser_for_branch(output, rules, &rule[0])?;
    if rule.len() == 2 {
        output.write_all(b",")?;
        parser_for_branch(output, rules, &rule[1])?;
        output.write_all(b"))")?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (_, rules) = many1(terminated(parse_rule, line_ending))(&input).unwrap();
    let rules = rules.into_iter().collect::<HashMap<_, _>>();

    let output = Path::new(&env::var("OUT_DIR").unwrap()).join("parser.rs");
    let mut output = File::create(output)?;
    parser_for_rule(&mut output, &rules, 0)?;

    let input = "0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"

aab
aba";
    let (_, rules) = many1(terminated(parse_rule, line_ending))(input)?;
    let rules = rules.into_iter().collect::<HashMap<_, _>>();

    let output = Path::new(&env::var("OUT_DIR").unwrap()).join("test1.rs");
    let mut output = File::create(output)?;
    parser_for_rule(&mut output, &rules, 0)?;

    Ok(())
}

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{pair, terminated},
    IResult,
};
use std::{collections::HashMap, fs, str::FromStr};

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

fn matches_component<'a>(
    message: &'a str,
    rules: &HashMap<usize, Rule>,
    component: &Component,
    last: bool,
) -> Result<Vec<&'a str>> {
    match component {
        Component::Reference(r) => matches_rule(message, rules, *r, last),
        Component::Literal(c) => match message.chars().next() {
            None => Err(anyhow!("no message remaining")),
            Some(_) if last && !message[1..].is_empty() => Err(anyhow!("message not done yet")),
            Some(m) if m == *c => Ok(vec![&message[1..]]),
            Some(_) => Err(anyhow!("no match")),
        },
    }
}

fn matches_branch<'a>(
    message: &'a str,
    rules: &HashMap<usize, Rule>,
    branch: &[Component],
    last: bool,
) -> Result<Vec<&'a str>> {
    let last_index = branch.len() - 1;
    branch
        .iter()
        .enumerate()
        .try_fold(vec![message], |messages, (index, component)| {
            let results = messages
                .iter()
                .filter_map(|message| {
                    matches_component(message, rules, component, last && last_index == index).ok()
                })
                .flatten()
                .collect::<Vec<_>>();
            if results.is_empty() {
                Err(anyhow!("no options matched"))
            } else {
                Ok(results)
            }
        })
}

fn matches_rule<'a>(
    message: &'a str,
    rules: &HashMap<usize, Rule>,
    rule_number: usize,
    last: bool,
) -> Result<Vec<&'a str>> {
    let rule = &rules[&rule_number];
    let matching_branches = rule
        .iter()
        .filter_map(|branch| matches_branch(message, rules, branch, last).ok())
        .flatten()
        .collect::<Vec<_>>();
    if matching_branches.is_empty() {
        Err(anyhow!("no branches matched"))
    } else {
        Ok(matching_branches)
    }
}

fn do_the_thing(input: &str) -> usize {
    let (input, rules) = many1(terminated(parse_rule, line_ending))(&input).unwrap();
    let rules = rules.into_iter().collect::<HashMap<_, _>>();
    let (messages, _) = line_ending::<&str, nom::error::Error<&str>>(input).unwrap();
    messages
        .lines()
        .filter(|message| matches_rule(message, &rules, 0, true).is_ok())
        .count()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input));

    let input = fs::read_to_string("input2.txt")?;
    println!("{:?}", do_the_thing(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"

aab
aba
ab
aa" => 2; "simplest example")]
    #[test_case(r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"# => 3; "part 2 without looping")]
    #[test_case(r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31 | 42 11 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42 | 42 8
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"# => 12; "part 2 example looping")]
    fn first(input: &str) -> usize {
        do_the_thing(&input)
    }
}

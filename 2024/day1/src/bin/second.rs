use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    multi::fold_many1,
    IResult,
};
use std::{collections::HashMap, fs, ops::AddAssign};

fn parse_line(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, a) = i64(input)?;
    let (input, _) = tag("   ")(input)?;
    let (input, b) = i64(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, (a, b)))
}

fn parse_columns(input: &str) -> IResult<&str, (Vec<i64>, HashMap<i64, i64>)> {
    fold_many1(
        parse_line,
        || (Vec::new(), HashMap::<i64, i64>::new()),
        |(mut col_a, mut col_b), (a, b)| {
            col_a.push(a);
            col_b.entry(b).or_default().add_assign(1);
            (col_a, col_b)
        },
    )(input)
}

fn run(input: &str) -> i64 {
    let (input, (column_a, column_b)) = parse_columns(input).unwrap();
    assert_eq!(input, "");
    column_a
        .into_iter()
        .map(|a| a * column_b.get(&a).unwrap_or(&0))
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("3   4
4   3
2   5
1   3
3   9
3   3
" => 31)]
    fn test(input: &str) -> i64 {
        run(input)
    }
}

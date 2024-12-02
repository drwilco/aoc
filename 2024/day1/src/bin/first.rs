use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    multi::fold_many1,
    IResult,
};
use std::fs;

fn parse_line(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, a) = i64(input)?;
    let (input, _) = tag("   ")(input)?;
    let (input, b) = i64(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, (a, b)))
}

fn parse_columns(input: &str) -> IResult<&str, (Vec<i64>, Vec<i64>)> {
    fold_many1(
        parse_line,
        || (Vec::new(), Vec::new()),
        |(mut col_a, mut col_b), (a, b)| {
            col_a.push(a);
            col_b.push(b);
            (col_a, col_b)
        },
    )(input)
}

fn run(input: &str) -> i64 {
    let (input, (mut column_a, mut column_b)) = parse_columns(input).unwrap();
    assert_eq!(input, "");
    column_a.sort_unstable();
    column_b.sort_unstable();
    column_a
        .into_iter()
        .zip(column_b)
        .map(|(a, b)| (a - b).abs())
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
" => 11)]
    fn test(input: &str) -> i64 {
        run(input)
    }
}

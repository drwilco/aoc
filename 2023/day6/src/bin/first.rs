use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    IResult,
};

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map(digit1, |s: &str| s.parse::<i64>().unwrap())(input)
}

fn parse_line(start_tag: &str) -> impl Fn(&str) -> IResult<&str, Vec<i64>> + '_ {
    move |input: &str| {
        let (input, _) = tag(start_tag)(input)?;
        let (input, _) = space1(input)?;
        let (input, numbers) = separated_list1(space1, parse_i64)(input)?;
        let (input, _) = line_ending(input)?;
        Ok((input, numbers))
    }
}

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn solutions(&self) -> impl Iterator<Item=i64> + '_ {
        (0..self.time)
           .filter(|speed| {
            let remaining_time = self.time - speed;
            let distance = speed * remaining_time;
            distance > self.distance
        })
    }
}

pub fn run(input: &str) -> i64 {
    let (input, times) = parse_line("Time:")(input).unwrap();
    let (input, distances) = parse_line("Distance:")(input).unwrap();
    assert!(input.is_empty());
    let races = times
        .into_iter()
        .zip(
            distances
        )
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<_>>();
    races.into_iter()
        .map(|r| r.solutions().count() as i64)
        .product()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Time:      7  15   30
Distance:  9  40  200
" => 288)]
    fn test(input: &str) -> i64 {
        run(&input)
    }
}

use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    IResult,
};

use rayon::prelude::*;

fn parse_line(start_tag: &str) -> impl Fn(&str) -> IResult<&str, i64> + '_ {
    move |input: &str| {
        let (input, _) = tag(start_tag)(input)?;
        let (input, _) = space1(input)?;
        let (input, numbers) = separated_list1(space1, digit1)(input)?;
        let number = numbers
            .into_iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let (input, _) = line_ending(input)?;
        Ok((input, number))
    }
}

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn solutions(&self) -> i64 {
        let first = (0..self.time).into_par_iter().find_first(|speed| {
            let remaining_time = self.time - speed;
            let distance = speed * remaining_time;
            distance > self.distance
        }).unwrap();
        let last = (first..self.time).into_par_iter().find_last(|speed| {
            let remaining_time = self.time - speed;
            let distance = speed * remaining_time;
            distance > self.distance
        }).unwrap();
        last - first + 1
    }
}

pub fn run(input: &str) -> i64 {
    let (input, time) = parse_line("Time:")(input).unwrap();
    let (input, distance) = parse_line("Distance:")(input).unwrap();
    assert!(input.is_empty());
    Race { time, distance }.solutions()
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
" => 71503)]
    fn test(input: &str) -> i64 {
        run(&input)
    }
}

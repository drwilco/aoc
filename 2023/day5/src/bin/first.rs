use std::{collections::HashMap, fs, ops::Range};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::map,
    error::Error,
    multi::{many1, separated_list1},
    IResult,
};

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map(digit1, |s: &str| s.parse::<i64>().unwrap())(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(nom::character::complete::space1, parse_i64)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, seeds))
}

#[derive(Debug)]
struct RangeMap {
    range: Range<i64>,
    offset: i64,
}

#[derive(Debug)]
struct CategoryMap {
    from: String,
    to: String,
    maps: Vec<RangeMap>,
}

impl CategoryMap {
    fn map(&self, value: i64) -> i64 {
        self.maps
            .iter()
            .find(|m| m.range.contains(&value))
            .map(|r| (value + r.offset))
            .unwrap_or(value)
    }
}

fn parse_range_map(input: &str) -> IResult<&str, RangeMap> {
    let (input, destination_range_start) = parse_i64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, source_range_start) = parse_i64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, range_length) = parse_i64(input)?;
    let (input, _) = newline(input)?;
    Ok((
        input,
        RangeMap {
            range: source_range_start..(source_range_start + range_length),
            offset: destination_range_start - source_range_start,
        },
    ))
}

fn parse_map(input: &str) -> IResult<&str, CategoryMap> {
    let (input, from) = alpha1(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, to) = alpha1(input)?;
    let (input, _) = tag(" map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, maps) = many1(parse_range_map)(input)?;
    Ok((
        input,
        CategoryMap {
            from: from.to_string(),
            to: to.to_string(),
            maps,
        },
    ))
}

pub fn run(input: &str) -> i64 {
    let (input, seeds) = parse_seeds(input).unwrap();
    let (input, _) = newline::<&str, Error<&str>>(input).unwrap();
    let (input, maps) = separated_list1(newline, parse_map)(input).unwrap();
    assert!(input.is_empty());
    let maps: HashMap<String, CategoryMap> =
        maps.into_iter().map(|m| (m.from.clone(), m)).collect();
    seeds
        .into_iter()
        .map(|mut id| {
            let mut category = "seed";
            while category != "location" {
                let map = &maps[category];
                let new_id = map.map(id);
                category = &map.to;
                id = new_id;
            }
            id
        })
        .min()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
" => 35)]
    fn test(input: &str) -> i64 {
        run(&input)
    }
}

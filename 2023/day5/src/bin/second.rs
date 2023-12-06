use std::{collections::HashMap, fs, ops::Range};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space1},
    combinator::map,
    error::Error,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

use itertools::Itertools;

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map(digit1, |s: &str| s.parse::<i64>().unwrap())(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<Range<i64>>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, ranges) = separated_list1(
        space1,
        map(
            separated_pair(parse_i64, space1, parse_i64),
            |(start, length)| start..(start + length),
        ),
    )(input)?;
    let (input, _) = newline(input)?;
    Ok((input, ranges))
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
    fn map(&self, mut values: Range<i64>) -> Vec<Range<i64>> {
        let mut result = Vec::new();
        while values.start < values.end {
            let map = self.maps.iter().find(|m| m.range.contains(&values.start));
            match map {
                None => {
                    // find first map that starts after values.start
                    let map = self.maps.iter().find(|m| m.range.start > values.start);
                    match map {
                        None => {
                            // no more maps, so just add the rest of the values
                            result.push(values.start..values.end);
                            break;
                        }
                        Some(map) => {
                            // check whether our values end before the start of
                            // the map
                            if values.end < map.range.start {
                                // add the rest of the values
                                result.push(values.start..values.end);
                                break;
                            }
                            // add the values up to the start of the map
                            result.push(values.start..map.range.start);
                            values.start = map.range.start;
                        }
                    }
                }
                Some(map) => {
                    // check if the end of our values is within the map
                    if map.range.end < values.end {
                        // add the values up to the end of the map, adding the offset
                        result.push((values.start + map.offset)..(map.range.end + map.offset));
                        // set the start of the next values to the end of the map
                        values.start = map.range.end;
                    } else {
                        // add the values up to the end of our values, adding the offset
                        result.push((values.start + map.offset)..(values.end + map.offset));
                        break;
                    }
                }
            }
        }
        result
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
    let (input, mut maps) = many1(parse_range_map)(input)?;
    // Sort them so the search in map() works
    maps.sort_unstable_by_key(|m| m.range.start);
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
    let mut category = "seed";
    let mut id_ranges = seeds;
    while category != "location" {
        let map = &maps[category];
        id_ranges = id_ranges
            .into_iter()
            .flat_map(|range| map.map(range))
            .collect();
        // Merge adjoining ranges for a minor speedup
        id_ranges.sort_unstable_by_key(|r| r.start);
        id_ranges = id_ranges
            .into_iter()
            .coalesce(|a, b| {
                if a.end == b.start {
                    Ok(a.start..b.end)
                } else {
                    Err((a, b))
                }
            })
            .collect();
        category = &map.to;
    }
    id_ranges
        .into_iter()
        .filter_map(|r| if r.start < r.end { Some(r.start) } else { None })
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
" => 46)]
    fn test(input: &str) -> i64 {
        run(&input)
    }
}

use std::fs;

use nom::{
    bytes::complete::tag,
    character::streaming::{i64 as parse_i64, line_ending},
    multi::many1,
    IResult,
};

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Pair {
    sensor: Point,
    beacon: Point,
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = parse_i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = parse_i64(input)?;
    Ok((input, Point { x, y }))
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor) = parse_point(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, beacon) = parse_point(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Pair { sensor, beacon }))
}

struct RangeInclusive<T> {
    start: T,
    end: T,
}

fn do_the_thing(input: &str, y: i64) -> usize {
    let (input, pairs) = many1(parse_pair)(input).unwrap();
    assert!(input.is_empty());

    let mut x_coords = pairs
        .iter()
        .filter_map(|p| {
            // first of all, only consider pairs in range of our Y coordinate
            // example beacon = (0,0), sensor = (1,0), y = -1 to y = 1 is in range.
            let distance = p.sensor.manhattan_distance(&p.beacon);
            if !((p.sensor.y - distance)..=(p.sensor.y + distance)).contains(&y) {
                return None;
            }
            // now find all the X coordinates of points on the Y coordinate that are
            // in range example beacon = (0,0), sensor = (2,0), y = 1 would have 3
            // poimts in range of the beacon on y = 1: (-1,1), (0,1), (1,1)
            // distance would be 2, remainder would be 1
            let remainder = distance - (y - p.sensor.y).abs();
            let mut start = p.sensor.x - remainder;
            let mut end = p.sensor.x + remainder;
            // Don't include the beacon if it is on our Y coordinate. If it is
            // on our Y, it will be either the min or max for this pair, so we
            // can just change those
            if start == p.beacon.x {
                start += 1;
            }
            if end == p.beacon.x {
                end -= 1;
            }
            Some(RangeInclusive { start, end })
        })
        .collect::<Vec<RangeInclusive<i64>>>();
    x_coords.sort_unstable_by_key(|r| r.start);
    x_coords
        .into_iter()
        .fold(Vec::<RangeInclusive<i64>>::new(), |mut acc, r| {
            if let Some(last) = acc.last_mut() {
                // Options are:
                // Consecutive ranges: 1..=2, 3..=4 -> 1..=4
                // Overlapping ranges: 1..=2, 2..=3 -> 1..=3
                // Completely contained: 1..=3, 2..=2 -> 1..=3
                // Completely separate: 1..=2, 4..=5 -> 1..=2, 4..=5

                // This only works because the ranges are sorted by start
                if last.end + 1 >= r.start {
                    // This should cover all except completely separate
                    last.end = last.end.max(r.end);
                    return acc;
                }
            }
            acc.push(r);
            acc
        })
        .into_iter()
        .map(|r| (r.end - r.start + 1) as usize)
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input, 2000000));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
", 10 => 26)]
    fn test(input: &str, y: i64) -> usize {
        do_the_thing(&input, y)
    }
}

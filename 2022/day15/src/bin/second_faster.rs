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

#[derive(Clone, Copy, Debug)]
struct RangeInclusive<T> {
    start: T,
    end: T,
}

fn do_the_thing(input: &str, max_xy: i64) -> i64 {
    let (input, pairs) = many1(parse_pair)(input).unwrap();
    assert!(input.is_empty());

    // Avoid allocations by reusing the same Vecs
    let mut x_coords: Vec<RangeInclusive<i64>> = Vec::new();
    let mut x_coords_merged: Vec<RangeInclusive<i64>> = Vec::new();
    for y in 0..=max_xy {
        // Could save one iteration of clear()s by doing it later, but this is
        // much easier to read
        x_coords.clear();
        x_coords_merged.clear();
        // For loop instead of filter_map(), for speed
        for p in &pairs {
            // first of all, only consider pairs in range of our Y coordinate
            // example beacon = (0,0), sensor = (1,0), y = -1 to y = 1 is in range.
            let distance = p.sensor.manhattan_distance(&p.beacon);
            if !((p.sensor.y - distance)..=(p.sensor.y + distance)).contains(&y) {
                continue;
            }
            // now find all the X coordinates of points on the Y coordinate that are
            // in range example beacon = (0,0), sensor = (2,0), y = 1 would have 3
            // poimts in range of the beacon on y = 1: (-1,1), (0,1), (1,1)
            // distance would be 2, remainder would be 1
            let remainder = distance - (y - p.sensor.y).abs();
            let mut start = p.sensor.x - remainder;
            let mut end = p.sensor.x + remainder;
            // constrain both start and end to 0 and max_xy
            start = start.max(0).min(max_xy);
            end = end.max(0).min(max_xy);
            // No longer filter known beacons, because those now count
            // as "can't be what we're looking for"
            x_coords.push(RangeInclusive { start, end });
        }
        x_coords.sort_unstable_by_key(|r| r.start);
        for r in x_coords.iter() {
            if let Some(last) = x_coords_merged.last_mut() {
                // Options are:
                // Consecutive ranges: 1..=2, 3..=4 -> 1..=4
                // Overlapping ranges: 1..=2, 2..=3 -> 1..=3
                // Completely contained: 1..=3, 2..=2 -> 1..=3
                // Completely separate: 1..=2, 4..=5 -> 1..=2, 4..=5

                // This only works because the ranges are sorted by start
                if last.end + 1 >= r.start {
                    // This should cover all except completely separate
                    last.end = last.end.max(r.end);
                    continue;
                }
            }
            x_coords_merged.push(*r);
        }
        // This is the most common case, so we check it first
        if x_coords_merged[0].start == 0 && x_coords_merged[0].end == max_xy {
            // This is not the row we're looking for
            continue;
        }
        let x = match x_coords_merged.len() {
            1 => {
                // Since there's only one gap, that means the gap is either at
                // the start or the end of the row
                if x_coords_merged[0].start == 0 {
                    // If we start at 0, then max_xy is the only possible value
                    max_xy
                } else {
                    // If we don't start at 0, x == 0
                    0
                }
            }
            2 => {
                // x is 1 more than end of first range
                x_coords_merged[0].end + 1
            }
            _ => panic!("too many ranges"),
        };
        return x * 4000000 + y;
    }
    panic!("no answer found");
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input, 4000000));
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
", 20 => 56000011)]
    fn test(input: &str, max_xy: i64) -> i64 {
        do_the_thing(&input, max_xy)
    }
}

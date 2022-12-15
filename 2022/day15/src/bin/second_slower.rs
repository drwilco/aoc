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

#[derive(Clone, Debug)]
struct RangeInclusive<T> {
    start: T,
    end: T,
}

fn do_the_thing(input: &str, max_xy: i64) -> i64 {
    let start = std::time::Instant::now();
    let (input, pairs) = many1(parse_pair)(input).unwrap();
    assert!(input.is_empty());
    println!("Parsing took {:?}", start.elapsed());

    let start = std::time::Instant::now();
    // Start with 0..=max_xy empty Vecs
    let mut ranges: Vec<Vec<RangeInclusive<i64>>> = Vec::with_capacity(max_xy as usize + 1);
    ranges.resize_with((max_xy + 1) as usize, Vec::new);
    println!("Making Vecs took {:?}", start.elapsed());

    let start = std::time::Instant::now();
    for pair in pairs {
        let distance = pair.sensor.manhattan_distance(&pair.beacon);
        // Find all the Y coordinates that are in range of the sensor
        let y_start = pair.sensor.y - distance;
        let y_end = pair.sensor.y + distance;
        // Constrain both start and end to 0 and max_xy
        let y_start = y_start.max(0).min(max_xy);
        let y_end = y_end.max(0).min(max_xy);
        for y in y_start..=y_end {
            // Now find all the X coordinates of points on the Y coordinate that are
            // in range example beacon = (0,0), sensor = (2,0), y = 1 would have 3
            // poimts in range of the beacon on y = 1: (-1,1), (0,1), (1,1)
            // distance would be 2, remainder would be 1
            let remainder = distance - (y - pair.sensor.y).abs();
            let mut x_start = pair.sensor.x - remainder;
            let mut x_end = pair.sensor.x + remainder;
            // Constrain both start and end to 0 and max_xy
            x_start = x_start.max(0).min(max_xy);
            x_end = x_end.max(0).min(max_xy);
            // No longer filter known beacons, because those now count
            // as "can't be what we're looking for"

            // Add the range to the list of ranges for this Y coordinate
            ranges[y as usize].push(RangeInclusive {
                start: x_start,
                end: x_end,
            });
        }
    }
    let duration = start.elapsed();
    println!("Time to build ranges: {:?}", duration);

    let start = std::time::Instant::now();
    // Merge all the ranges for each Y coordinate, use find to return early if we find a row with a gap
    let (x, y) = ranges
        .into_iter()
        .enumerate()
        .find_map(|(y, mut x_coords)| {
            x_coords.sort_unstable_by_key(|r| r.start);
            x_coords =
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
                    });
            match x_coords.len() {
                1 => {
                    if x_coords[0].start == 0 && x_coords[0].end == max_xy {
                        // This is not the row we're looking for
                        None
                    } else {
                        // If we start at 0, then max_xy is the only possible value
                        Some((
                            if x_coords[0].start == 0 {
                                max_xy
                            } else {
                                // If we don't start at 0, x == 0
                                0
                            },
                            y,
                        ))
                    }
                }
                2 => {
                    // x is 1 more than end of first range
                    Some(((x_coords[0].end + 1), y))
                }
                _ => panic!("too many ranges"),
            }
        })
        .expect("didn't find a row");

    let duration = start.elapsed();
    println!("Time to find row while merging: {:?}", duration);
    x * 4000000 + (y as i64)
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

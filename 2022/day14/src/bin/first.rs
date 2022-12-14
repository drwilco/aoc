use std::{collections::HashSet, fs};

use nom::{
    bytes::complete::tag,
    character::complete::{char, i32 as parse_i32, line_ending},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

// To start we don't need to track the difference between resting sand and rock
// if we track how many times falling sand has become resting sand. So we only
// need to track blocking or not.
//
// A couple of possibilities for the datastructure to track the environment
// 1. A 2D array (Vec of Vecs) of bools, where true means blocking.
// 2. A HashSet of Points, where any existing Point is blocking.
// 3. A Vec of Vec of Points, where the Vec of Points are columns
//
// #1 is easy to implement, but we need to know the size of the environment,
// plus the sand can pile up above the highest rock point. We would have to scan
// the source column to find the highest blocked point, unless we track that...
// We can determine the highest possible pile by taking min/max X and going up
// diagonally from there to the source column.
//
// #2 is also pretty easy, we wouldn't need to know the size of the environment
// and the same applies about tracking the highest point as above.
//
// #3 would allow us to have the highest point in the source column be the last
// element, but going sideways there might be rock above, and that would mean
// we'd have to scan the column, and insert between, which gets really messy, so
// #3 is completely out.
//
// So, #2 it is. But I think we'll keep a separate HashMap of Point -> last
// impact Point for optimization purposes. We'll have at least one for the
// source column, and then add more for drops from edges. Or at least something
// like that.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, x) = parse_i32(input)?;
    let (input, _) = char(',')(input)?;
    let (input, y) = parse_i32(input)?;
    Ok((input, Point { x, y }))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Point>> {
    terminated(separated_list1(tag(" -> "), parse_point), line_ending)(input)
}

fn do_the_thing(input: &str) -> usize {
    let (input, lines) = many1(parse_line)(input).unwrap();
    assert!(input.is_empty());

    // Fill the environment with our lines
    //
    // Y goes down, X goes right. The void is below the lowest point so track
    // max Y. Min Y is interesting for the first time doing the source column,
    // but maybe that's overkill for now. We don't need min/max X, as we're
    // using the HashSet approach, and don't need to know how wide things are.
    //
    // Accessing the outer fold's variables directly from the inner fold is a
    // bit dirty, but fine for now.
    let (mut environment, max_y) = lines.into_iter().fold(
        (HashSet::<Point>::new(), None::<i32>),
        |(mut environment, max_y), line| {
            let (new_points, max_y, _) = line.into_iter().fold(
                (Vec::<Point>::new(), max_y, None::<Point>),
                |(mut acc, mut max_y, prev), point| {
                    if let Some(prev) = prev {
                        for x in if prev.x < point.x {
                            prev.x..=point.x
                        } else {
                            point.x..=prev.x
                        } {
                            for y in if prev.y < point.y {
                                prev.y..=point.y
                            } else {
                                point.y..=prev.y
                            } {
                                // lines of 2 or more segments will insert start of
                                // line over the end of the previous line, but
                                // that's fine. It's more code and probably more CPU
                                // work to check for that. HashSet will do the work.
                                acc.push(Point { x, y });
                            }
                        }
                    }
                    if let Some(y) = max_y {
                        max_y = Some(y.max(point.y));
                    } else {
                        max_y = Some(point.y);
                    }
                    (acc, max_y, Some(point))
                },
            );
            environment.extend(new_points.into_iter());
            (environment, max_y)
        },
    );

    // we should have at least one line, so max_y should be set
    let max_y = max_y.unwrap();

    // loop until sand drops below max Y, then return out
    let mut resting_sand: usize = 0;
    loop {
        // Source is a 500,0
        let mut sand = Point { x: 500, y: 0 };
        loop {
            // Whether or not we've dropped down at all in this
            // iteration of the inner loop
            let mut dropped = false;

            // try straight down first, then left-down, then right-down
            for dx in [0, -1, 1] {
                let next = Point {
                    x: sand.x + dx,
                    y: sand.y + 1,
                };
                // if we can drop down to said position, do so
                if !environment.contains(&next) {
                    // next spot is empty, and it's at max Y, so there can't be
                    // anything blocking us below, so we're done
                    if next.y >= max_y {
                        return resting_sand;
                    }
                    sand = next;
                    dropped = true;
                    break;
                }
            }
            if !dropped {
                environment.insert(sand);
                resting_sand += 1;
                break;
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
" => 24)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

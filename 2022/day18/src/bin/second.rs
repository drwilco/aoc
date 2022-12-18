use std::{collections::HashSet, fs, ops::Add};

use lazy_static::lazy_static;

use nom::{
    character::complete::{char, i32 as parse_i32, line_ending},
    multi::many1,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, x) = parse_i32(input)?;
    let (input, _) = char(',')(input)?;
    let (input, y) = parse_i32(input)?;
    let (input, _) = char(',')(input)?;
    let (input, z) = parse_i32(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Point { x, y, z }))
}

const SIDES: [Point; 6] = [
    Point { x: 1, y: 0, z: 0 },
    Point { x: -1, y: 0, z: 0 },
    Point { x: 0, y: 1, z: 0 },
    Point { x: 0, y: -1, z: 0 },
    Point { x: 0, y: 0, z: 1 },
    Point { x: 0, y: 0, z: -1 },
];

lazy_static! {
    static ref NEIGHBORS: Vec<Point> = {
        let mut neighbors = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if !(x == 0 && y == 0 && z == 0) {
                        neighbors.push(Point { x, y, z });
                    }
                }
            }
        }
        neighbors
    };
}

impl Point {
    fn neighbors(&self) -> impl Iterator<Item = Point> + '_ {
        NEIGHBORS.iter().map(move |neighbor| self + neighbor)
    }
    fn sides(&self) -> impl Iterator<Item = Point> + '_ {
        SIDES.iter().map(move |side| self + side)
    }
}

fn connected_empty_layer(solid: HashSet<Point>, empty_spot: &Point) -> HashSet<Point> {
    assert!(!solid.contains(empty_spot));
    assert!(SIDES
        .iter()
        .any(|side| solid.contains(&(empty_spot + side))));
    let mut layer = HashSet::new();
    let mut queue = vec![*empty_spot];
    while let Some(point) = queue.pop() {
        for neighbor in point.sides() {
            // We're only interested in empty spaces
            if solid.contains(&neighbor) {
                continue;
            }
            // But they have to be next to a solid space
            if neighbor.neighbors().any(|nn| solid.contains(&nn)) {
                // if it's new to the layer, add it to the queue
                if layer.insert(neighbor) {
                    queue.push(neighbor);
                }
            }
        }
    }
    layer
}

fn do_the_thing(input: &str) -> usize {
    let (input, solid) = many1(parse_point)(input).unwrap();
    assert!(input.is_empty());

    let solid: HashSet<Point> = solid.into_iter().collect();

    // get groups of empty spots inside and out
    let mut empty_layers: Vec<HashSet<Point>> = Vec::new();
    for point in solid.iter() {
        for side in point.sides() {
            // It has to be empty
            if !solid.contains(&side) {
                // And not already in a layer
                if empty_layers.iter().any(|layer| layer.contains(&side)) {
                    continue;
                }
                empty_layers.push(connected_empty_layer(solid.clone(), &side));
            }
        }
    }

    println!("{} empty layers", empty_layers.len());
    // The largest empty layer is around the outside
    empty_layers.sort_by_key(|layer| layer.len());
    let outside = empty_layers.pop().unwrap();

    // Get all the sides that are in the outside layer
    solid
        .into_iter()
        .map(|point| point.sides().filter(|side| outside.contains(side)).count())
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
" => 58)]
    #[test_case("1,1,1
2,1,1
" => 10)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

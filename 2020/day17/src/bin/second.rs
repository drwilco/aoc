#[macro_use]
extern crate lazy_static;

use anyhow::{Error, Result};
use itertools::Itertools;
use std::{collections::HashSet, fs, ops::Add, str::FromStr};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Add for &Point {
    type Output = Point;
    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
struct Space(HashSet<Point>);

lazy_static! {
    static ref NEIGHBOR_OFFSETS: Vec<Point> = {
        let offsets: Vec<isize> = vec![-1, 0, 1];
        offsets
            .iter()
            .cartesian_product(offsets.iter())
            .cartesian_product(offsets.iter())
            .cartesian_product(offsets.iter())
            .filter_map(|(((&x, &y), &z), &w)| {
                if x == 0 && y == 0 && z == 0 && w == 0 {
                    None
                } else {
                    Some(Point { x, y, z, w })
                }
            })
            .collect()
    };
}

impl FromStr for Space {
    type Err = Error;

    fn from_str(input: &str) -> Result<Space> {
        Ok(Space(
            input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| {
                        if c == '#' {
                            Some(Point {
                                x: x as isize,
                                y: y as isize,
                                z: 0,
                                w: 0,
                            })
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        ))
    }
}

impl Space {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn inactive_neighbors(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        for active in &self.0 {
            for offset in NEIGHBOR_OFFSETS.iter() {
                let potential = offset + active;
                if self.0.get(&potential).is_none() {
                    result.insert(potential);
                }
            }
        }
        result
    }

    fn active_neighbor_count(&self, point: &Point) -> usize {
        NEIGHBOR_OFFSETS
            .iter()
            .filter(|offset| self.0.get(&(point + offset)).is_some())
            .count()
    }
}

fn do_the_thing(input: &str, cycles: usize) -> Result<usize> {
    let mut space = Space::from_str(input)?;
    for _ in 0..cycles {
        space = Space({
            let from_inactive = space
                .inactive_neighbors()
                .into_iter()
                .filter(|point| space.active_neighbor_count(point) == 3);
            let from_active = space
                .0
                .iter()
                .filter(|point| {
                    let count = space.active_neighbor_count(point);
                    count == 2 || count == 3
                })
                .cloned();
            from_active.chain(from_inactive).collect()
        });
    }
    Ok(space.len())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input, 6)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0 => 5)]
    #[test_case(6 => 848)]
    fn second(cycles: usize) -> usize {
        let input = ".#.
..#
###";
        do_the_thing(input, cycles).unwrap()
    }
}

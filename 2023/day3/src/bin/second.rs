use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::is_a,
    character::complete::{digit1, none_of},
    combinator::map,
    multi::many1,
    IResult,
};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Component {
    Part(String),
    Symbol(char),
    Empty(usize),
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn neighbors(&self, len: usize) -> Vec<Point> {
        let len = len as i32;
        let mut neighbors = vec![
            Point { x: self.x - 1, y: self.y },
            Point { x: self.x - 1, y: self.y - 1 },
            Point { x: self.x - 1, y: self.y + 1 },
            Point { x: self.x + len, y: self.y },
            Point { x: self.x + len, y: self.y - 1 },
            Point { x: self.x + len, y: self.y + 1 },
        ];
        for i in 0..len {
            neighbors.push(Point { x: self.x + i, y: self.y - 1 });
            neighbors.push(Point { x: self.x + i, y: self.y + 1 });
        }
        neighbors
    }
}

fn parse_component(input: &str) -> IResult<&str, Component> {
    alt((
        map(digit1, |s: &str| Component::Part(s.to_string())),
        map(none_of("1234567890."), Component::Symbol),
        map(is_a("."), |s: &str| Component::Empty(s.len())),
    ))(input)
}

fn do_the_thing(input: &str) -> usize {
    let mut map = HashMap::<Point, Component>::new();
    for (row, line) in input.lines().enumerate() {
        let components = many1(parse_component)(line).unwrap().1;
        let mut col = 0;
        for component in components {
            match component {
                Component::Part(ref s) => {
                    for _ in 0..s.len() {                
                        map.insert(Point { x: col as i32, y: row as i32 }, component.clone());
                        col += 1;
                    }
                }
                Component::Symbol(_) => {
                    map.insert(Point { x: col as i32, y: row as i32 }, component.clone());
                    col += 1;
                }
                Component::Empty(n) => {
                    col += n;
                }
            }
        }
    }
    map
        .iter()
        .filter_map(|(point, component)| {
            if component != &Component::Symbol('*') {
                return None;
            }
            let mut neighboring_parts = point
                .neighbors(1)
                .into_iter()
                .filter_map(|point| {
                    match map.get(&point) {
                        Some(Component::Part(s)) => Some(s.parse::<usize>().unwrap()),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>();
            neighboring_parts[..].sort();
            // If there's any gear between same part numbers, this won't fly.
            // To fix that we would need to add a unique id to each part, and
            // dedup on that.
            neighboring_parts.dedup();
            if neighboring_parts.len() == 2 {
                return Some(neighboring_parts[0] * neighboring_parts[1]);
            }
            None
        }
    ).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
" => 467835)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

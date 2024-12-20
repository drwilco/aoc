#![feature(test)]

use std::{collections::HashMap, fs};

use itertools::Itertools;

#[derive(Debug, Default)]
struct FrequencyMaps {
    maps: HashMap<char, Vec<(i32, i32)>>,
    width: i32,
    height: i32,
}

fn parse_input(input: &str) -> FrequencyMaps {
    let mut freq_maps = FrequencyMaps::default();
    let mut width = None;
    for (y, line) in input.lines().enumerate() {
        let y = i32::try_from(y).unwrap();
        if width.is_none() {
            width = i32::try_from(line.len()).ok();
        }
        freq_maps.height += 1;
        for (x, character) in line.chars().enumerate() {
            let x = i32::try_from(x).unwrap();
            if !(character.is_ascii_alphanumeric()) {
                continue;
            }
            let map = freq_maps.maps.entry(character).or_default();
            map.push((x, y));
        }
    }
    freq_maps.width = width.unwrap();
    freq_maps
}

fn run(input: &str) -> usize {
    let freq_maps = parse_input(input);
    freq_maps
        .maps
        .into_iter()
        .flat_map(|(_, positions)| {
            positions
                .into_iter()
                .permutations(2)
                .filter_map(|positions| {
                    let (x1, y1) = positions[0];
                    let (x2, y2) = positions[1];
                    let (dx, dy) = (x2 - x1, y2 - y1);
                    let (anti_x, anti_y) = (x1 - dx, y1 - dy);
                    if anti_x >= 0
                        && anti_y >= 0
                        && anti_x < freq_maps.width
                        && anti_y < freq_maps.height
                    {
                        Some((anti_x, anti_y))
                    } else {
                        None
                    }
                })
        })
        .sorted()
        .dedup()
        .count()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    extern crate test as std_test;
    use super::*;
    use std_test::{black_box, Bencher};
    use test_case::test_case;

    #[bench]
    fn my_benchmark(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = black_box(&input);
        b.iter(|| run(input));
    }

    #[test_case("............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
" => 14)]
    fn test(input: &str) -> usize {
        run(input)
    }
}

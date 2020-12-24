use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{iterator, value},
    IResult,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse_direction(input: &str) -> IResult<&str, (isize, isize)> {
    alt((
        value((2, 0), tag("e")),
        value((1, -1), tag("ne")),
        value((1, 1), tag("se")),
        value((-2, 0), tag("w")),
        value((-1, -1), tag("nw")),
        value((-1, 1), tag("sw")),
    ))(input)
}

fn walk(input: &str) -> (isize, isize) {
    iterator(input, parse_direction).fold((0, 0), |(x, y), (x_offset, y_offset)| {
        (x + x_offset, y + y_offset)
    })
}

fn do_the_thing(input: &str, days: usize) -> usize {
    let mut black_tiles =
        input
            .lines()
            .map(|line| walk(line))
            .fold(HashSet::new(), |mut acc, tile| {
                if !acc.insert(tile) {
                    acc.remove(&tile);
                }
                acc
            });

    let neighbor_offsets = vec![(2, 0), (1, -1), (1, 1), (-2, 0), (-1, -1), (-1, 1)];
    for _ in 0..days {
        let new_black_tiles = black_tiles
            .iter()
            .cartesian_product(neighbor_offsets.iter())
            .filter_map(|((x, y), (x_offset, y_offset))| {
                let possible_white_tile = (x + x_offset, y + y_offset);
                if black_tiles.contains(&possible_white_tile) {
                    None
                } else {
                    Some(possible_white_tile)
                }
            })
            .fold(HashMap::new(), |mut acc, tile| {
                acc.entry(tile).and_modify(|count| *count += 1).or_insert(1);
                acc
            })
            .into_iter()
            .filter_map(|(tile, count)| if count == 2 { Some(tile) } else { None });
        let stay_black_tiles = black_tiles
            .iter()
            .filter(|(x, y)| {
                let count = neighbor_offsets
                    .iter()
                    .filter(|(x_offset, y_offset)| {
                        black_tiles.contains(&(x + x_offset, y + y_offset))
                    })
                    .count();
                count == 1 || count == 2
            })
            .copied();
        black_tiles = new_black_tiles.chain(stay_black_tiles).collect();
    }
    black_tiles.len()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input, 100));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0 => 10)]
    #[test_case(1 => 15)]
    #[test_case(2 => 12)]
    #[test_case(3 => 25)]
    #[test_case(4 => 14)]
    #[test_case(5 => 23)]
    #[test_case(100 => 2208)]
    fn first(days: usize) -> usize {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        do_the_thing(&input, days)
    }
    #[test]
    fn walker() {
        assert_eq!(walk("esew"), (1, 1));
        assert_eq!(walk("nwwswee"), (0, 0));
    }
}

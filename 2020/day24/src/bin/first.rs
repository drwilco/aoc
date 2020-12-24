use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{iterator, value},
    IResult,
};
use std::{collections::HashSet, fs};

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
fn do_the_thing(input: &str) -> usize {
    let black_tiles = input
        .lines()
        .map(|line| walk(line))
        .fold(HashSet::new(), |mut acc, tile| {
            if !acc.insert(tile) {
                acc.remove(&tile);
            }
            acc
        });
    black_tiles.len()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("sesenwnenenewseeswwswswwnenewsewsw
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
wseweeenwnesenwwwswnew" => 10)]
    fn first(input: &str) -> usize {
        do_the_thing(&input)
    }
    #[test]
    fn walker() {
        assert_eq!(walk("esew"), (1, 1));
        assert_eq!(walk("nwwswee"), (0, 0));
    }
}

#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{pair, preceded},
    IResult,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    ops::Add,
    str::FromStr,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for &Point {
    type Output = Point;
    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

lazy_static! {
    static ref NEIGHBOR_OFFSETS: Vec<Point> = vec![
        Point { x: 0, y: 1 },
        Point { x: 0, y: -1 },
        Point { x: 1, y: 0 },
        Point { x: -1, y: 0 },
    ];
}

struct Space(HashMap<Point, Tile>);

impl Space {
    fn potential_neighbors(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        for occupied in self.0.keys() {
            for offset in NEIGHBOR_OFFSETS.iter() {
                let potential = offset + occupied;
                if self.0.get(&potential).is_none() {
                    result.insert(potential);
                }
            }
        }
        result
    }

    fn fits(&self, location: &Point, candidate: &Tile) -> bool {
        if let Some(neighbor) = self.0.get(&(location + &Point { x: 1, y: 0 })) {
            let edges_match = candidate
                .pixels
                .iter()
                .zip(neighbor.pixels.iter())
                .fold(true, |acc, (c_row, n_row)| {
                    acc & (c_row.iter().next_back() == n_row.iter().next())
                });
            if !edges_match {
                return false;
            }
        }
        if let Some(neighbor) = self.0.get(&(location + &Point { x: -1, y: 0 })) {
            let edges_match = candidate
                .pixels
                .iter()
                .zip(neighbor.pixels.iter())
                .fold(true, |acc, (c_row, n_row)| {
                    acc & (c_row.iter().next() == n_row.iter().next_back())
                });
            if !edges_match {
                return false;
            }
        }
        if let Some(neighbor) = self.0.get(&(location + &Point { x: 0, y: 1 })) {
            if candidate.pixels.iter().next_back().unwrap()
                != neighbor.pixels.iter().next().unwrap()
            {
                return false;
            }
        }
        if let Some(neighbor) = self.0.get(&(location + &Point { x: 0, y: -1 })) {
            if candidate.pixels.iter().next().unwrap()
                != neighbor.pixels.iter().next_back().unwrap()
            {
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Debug)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<char>>,
}

fn parse_num<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    map(digit1, |digit_str: &str| digit_str.parse::<T>().unwrap())(input)
}

impl Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, id) = preceded(tag("Tile "), parse_num)(input)?;
        let (input, _) = pair(tag(":"), line_ending)(input)?;

        let (input, pixels) = separated_list1(line_ending, is_a("#."))(input)?;
        let pixels = pixels
            .into_iter()
            .map(|row| row.chars().collect())
            .collect();
        let (input, _) = pair(line_ending, line_ending)(input)?;
        Ok((input, Tile { id, pixels }))
    }
    fn rotated(&self) -> Self {
        // cols/rows is of the original, not the result. That's why the for
        // loops are weird.
        let cols = self.pixels[0].len();
        let rows = self.pixels.len();
        let y_max = cols - 1;
        let mut pixels = self.pixels.clone();
        for x in 0..rows {
            for y in 0..cols {
                pixels[y][x] = self.pixels[x][y_max - y];
            }
        }
        Self {
            id: self.id,
            pixels,
        }
    }
    fn flipped(&self) -> Self {
        let mut result = self.clone();
        result.pixels.reverse();
        result
    }
    fn _to_string(&self) -> String {
        self.pixels.iter().fold(String::new(), |mut acc, row| {
            acc.push_str(&row.iter().collect::<String>());
            acc.push('\n');
            acc
        })
    }
    fn mutations(&self) -> Mutations {
        Mutations {
            tile: self.clone(),
            mutations: 0,
        }
    }
}

struct Mutations {
    tile: Tile,
    mutations: usize,
}

impl Iterator for Mutations {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        self.mutations += 1;
        match self.mutations {
            0 => unreachable!(),
            1 => Some(self.tile.clone()),
            5 => {
                self.tile = self.tile.flipped();
                Some(self.tile.clone())
            }
            2 | 3 | 4 | 6 | 7 | 8 => {
                self.tile = self.tile.rotated();
                Some(self.tile.clone())
            }

            _ => None,
        }
    }
}

fn do_the_thing(input: &str) -> usize {
    let (_, mut tiles) = many1(Tile::parse)(input).unwrap();
    let mut grid: Space = Space(HashMap::default());
    grid.0.insert(Point { x: 0, y: 0 }, tiles.pop().unwrap());
    let mut tiles = VecDeque::from(tiles);

    'outer: while !tiles.is_empty() {
        let candidate = tiles.pop_front().unwrap();
        for potential_location in grid.potential_neighbors() {
            for candidate in candidate.mutations() {
                if grid.fits(&potential_location, &candidate) {
                    grid.0.insert(potential_location, candidate);
                    continue 'outer;
                }
            }
        }
        tiles.push_back(candidate);
    }
    let size = grid.0.get(&Point { x: 0, y: 0 }).unwrap().pixels.len();
    let new_size = size - 2;
    let grid_x_min = grid.0.keys().map(|p| p.x).min().unwrap();
    let grid_x_max = grid.0.keys().map(|p| p.x).max().unwrap();
    let grid_y_min = grid.0.keys().map(|p| p.y).min().unwrap();
    let grid_y_max = grid.0.keys().map(|p| p.y).max().unwrap();
    let mut pixels: Vec<Vec<char>> = Vec::new();
    for grid_y in grid_y_min..=grid_y_max {
        let mut rows: Vec<Vec<char>> = Vec::with_capacity(new_size);
        rows.resize_with(new_size, || Vec::default());
        for grid_x in grid_x_min..=grid_x_max {
            let tile = grid
                .0
                .get(&Point {
                    x: grid_x,
                    y: grid_y,
                })
                .unwrap();
            for (x, row) in rows.iter_mut().enumerate() {
                row.extend_from_slice(&tile.pixels[x + 1][1..=new_size]);
            }
        }
        pixels.append(&mut rows);
    }
    let size = pixels.len();
    let potential_roughness: usize = pixels
        .iter()
        .map(|line| line.iter().filter(|c| **c == '#').count())
        .sum();
    let tile = Tile { id: 0, pixels };

    let monster = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
    let monster_width = monster.lines().next().unwrap().len();
    let monster_points = monster
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Point {
                        x: x as isize,
                        y: y as isize,
                    })
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<_>>();
    let monster_points_len = monster_points.len();

    tile.mutations()
        .fold((0, 0), |(max_monsters, roughness), tile| {
            let mut monsters: usize = 0;
            for window in tile.pixels.windows(3) {
                for offset in 0..=(size - monster_width) {
                    let monster_present = monster_points
                        .iter()
                        .find(|p| window[p.y as usize][p.x as usize + offset] != '#')
                        .is_none();
                    if monster_present {
                        monsters += 1;
                    }
                }
            }
            if monsters > max_monsters {
                (
                    monsters,
                    potential_roughness - (monsters * monster_points_len),
                )
            } else {
                (max_monsters, roughness)
            }
        })
        .1
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

    #[test_case("Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...

" => 273; "example")]
    fn first(input: &str) -> usize {
        do_the_thing(&input)
    }
}

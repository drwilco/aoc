use rayon::prelude::*;
use std::char;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::ops;
use std::thread;
use std::time::Duration;
use std::usize;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Default)]
struct Point {
    x: isize,
    y: isize,
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Point {
        match rhs {
            Direction::North => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::East => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl ops::AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::North => self.y -= 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
            Direction::East => self.x += 1,
        };
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum TileType {
    Wall,
    Open,
    Start,
    Door(char),
    Key(char),
    Checked,
}

type TileMap = HashMap<Point, TileType>;
type MapIndex = HashMap<TileType, Point>;

fn string_to_data(string: &str) -> (TileMap, MapIndex, Vec<Point>) {
    let mut x = 0;
    let mut y = 0;
    let mut map: TileMap = HashMap::new();
    let mut index: MapIndex = HashMap::new();
    let mut start: Vec<Point> = Vec::new();
    for c in string.chars() {
        let point = Point { x, y };
        match c {
            '#' => {
                map.insert(point, TileType::Wall);
            }
            '.' => {
                map.insert(point, TileType::Open);
            }
            '@' => {
                map.insert(point, TileType::Open);
//                index.insert(TileType::Start, point);
                start.push(point);
            }
            'a'..='z' => {
                map.insert(point, TileType::Key(c));
                index.insert(TileType::Key(c), point);
            }
            'A'..='Z' => {
                map.insert(point, TileType::Door(c));
                index.insert(TileType::Door(c), point);
            }
            '\n' => (),
            _ => panic!("unexpected character: {:?}", c),
        }
        if c == '\n' {
            y += 1;
            x = 0;
        } else {
            x += 1;
        }
    }
    (map, index, start)
}

fn show_map(map: &TileMap) {
    println!();
    println!();
    let min_x = map.keys().map(|p| p.x).min().unwrap();
    let max_x = map.keys().map(|p| p.x).max().unwrap();
    let min_y = map.keys().map(|p| p.y).min().unwrap();
    let max_y = map.keys().map(|p| p.y).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let tile = map.get(&Point { x, y });
            let character = match tile {
                Some(TileType::Wall) => '#',
                Some(TileType::Open) => '.',
                Some(TileType::Key(c)) => *c,
                Some(TileType::Door(c)) => *c,
                Some(TileType::Checked) => ' ',
                Some(TileType::Start) => '@',
                None => ' ',
            };
            print!("{}", character);
        }
        println!();
    }
    //thread::sleep(Duration::from_millis(1000));
}

fn expand(
    map: &HashMap<Point, TileType>,
    mut point: Point,
    to: Direction,
    new_expand_from: &mut Vec<Point>,
    options: &mut Vec<(Point, usize)>,
    steps: usize,
) {
    point += to;
    match map.get(&point).unwrap() {
        TileType::Open => new_expand_from.push(point),
        TileType::Key(_) => options.push((point, steps)),
        _ => (),
    }
}

fn find_keys(map: &TileMap, start: Vec<Point>) -> Vec<(Point, usize)> {
    let mut map = map.clone();
    let mut options = Vec::new();
    let mut expand_from = start.to_vec();
    let mut steps = 0;
    while !expand_from.is_empty() {
        let mut new_expand_from = Vec::new();
        steps += 1;
        for point in expand_from.iter() {
            map.insert(*point, TileType::Checked);
            expand(&map, *point, Direction::North, &mut new_expand_from, &mut options, steps);
            expand(&map, *point, Direction::South, &mut new_expand_from, &mut options, steps);
            expand(&map, *point, Direction::East, &mut new_expand_from, &mut options, steps);
            expand(&map, *point, Direction::West, &mut new_expand_from, &mut options, steps);
        }
        expand_from.truncate(0);
        expand_from.append(&mut new_expand_from);
        //show_map(&map);
    }
    options
}

fn approx_min_steps(map: &TileMap, index: &MapIndex, start: Vec<Point>, steps: usize) -> usize {
    let mut keys = find_keys(map, start);
    if keys.is_empty() {
        panic!("should not happen");
    }
    keys.sort_by(|a, b| a.1.cmp(&b.1));
    let mut submap = map.clone();
    let mut index = index.clone();
    let keypos = keys[0].0;
    let keysteps = keys[0].1;
    let key = submap.insert(keypos, TileType::Open).unwrap();
    index.remove(&key).unwrap();
    if let TileType::Key(c) = key {
        let door = TileType::Door(c.to_uppercase().next().unwrap());
        if let Some(doorpos) = index.get(&door) {
            submap.insert(*doorpos, TileType::Open);
            index.remove(&door);
        }
    } else {
        panic!("somehow not a key");
    }
    if index.is_empty() {
        return steps + keysteps;
    }
    approx_min_steps(&submap, &index, keypos, steps + keysteps)
}

fn find_min_steps(map: &TileMap, index: &MapIndex, start: Vec<Point>, steps: usize, mut bound: usize, depth: usize) -> usize {
    let keys = find_keys(map, start);
    if keys.is_empty() {
        panic!("should not happen");
    }
    let approx = approx_min_steps(&map, &index, start, steps);
    if approx < bound {
        println!("approx bound adjust: {} < {}", approx, bound);
        bound = approx;
    }
    let mut min = usize::MAX;
    let mut minkeypos = keys[0].0;
    let mut minkeysteps = keys[0].1; 
//    println!("depth {} options {}", depth, keys.len());
    for (keypos, keysteps) in keys {
        if steps + keysteps > bound {
            continue;
        }
        let mut submap = map.clone();
        let mut index = index.clone();
        let key = submap.insert(keypos, TileType::Open).unwrap();
        index.remove(&key).unwrap();
        if let TileType::Key(c) = key {
            let door = TileType::Door(c.to_uppercase().next().unwrap());
            if let Some(doorpos) = index.get(&door) {
                submap.insert(*doorpos, TileType::Open);
                index.remove(&door);
            }
        } else {
            panic!("somehow not a key");
        }
        if index.is_empty() {
            println!("completed at {:?} steps", steps + keysteps);
            return steps + keysteps;
        }
        let result = approx_min_steps(&submap, &index, keypos, steps + keysteps);
        if result < bound {
            println!("new bound {:?} at depth {}", result, depth);
            bound = result;
        }
        if result < min {
            min = result;
            minkeypos = keypos;
            minkeysteps = keysteps;
        }
    }
    let mut submap = map.clone();
    let mut index = index.clone();
    let key = submap.insert(minkeypos, TileType::Open).unwrap();
    index.remove(&key).unwrap();
    if let TileType::Key(c) = key {
        let door = TileType::Door(c.to_uppercase().next().unwrap());
        if let Some(doorpos) = index.get(&door) {
            submap.insert(*doorpos, TileType::Open);
            index.remove(&door);
        }
    } else {
        panic!("somehow not a key");
    }
    find_min_steps(&submap, &index, minkeypos, steps + minkeysteps, bound, depth + 1)
}

fn actual_main(input: &str) -> usize {
    let (mut map, mut index, start) = string_to_data(input);
    let bound = approx_min_steps(&map, &index, start, 0);
    println!("bound: {}", bound);
    let result = find_min_steps(&mut map, &mut index, start, 0, bound, 0);
    println!("result: {:?}", result);
    result
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input2.txt")?;
    let result = actual_main(&input);
    println!("{:?}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case("#########
#b.A.@.a#
#########" => 8; "day 18 example 1")]
    #[test_case("########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################" => 86; "day 18 example 2")]
    #[test_case("########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################" => 132; "day 18 example 3")]
    #[test_case("#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################" => 136; "day 18 example 4")]
    #[test_case("########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################" => 81; "day 18 example 5")]
    fn test(input: &str) -> usize {
        actual_main(input)
    }
}

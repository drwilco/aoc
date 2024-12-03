use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
  x: isize,
  y: isize,
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

type Map = HashMap<Point, bool>;

struct MapMeta {
  width: isize,
  height: isize,
  map: Map,
}

fn _display_view(view: &Map, width: isize, height: isize) {
  for y in 0..height {
    for x in 0..width {
      match view.get(&Point{x, y}) {
        Some(_) => print!("#"),
        None => print!("."),
      }
    }
    println!("");
  }
  println!("");
}

fn string_to_map(input: &str) -> MapMeta {
  let mut map = HashMap::new();
  let height = input.lines().count() as isize;
  let width = input.lines().next().unwrap().len() as isize;
  for (y, line) in input.lines().enumerate() {
    let y = y as isize;
    for (x, character) in line.chars().enumerate() {
      let x = x as isize;
      if character == '#' {
        map.insert(Point{x, y}, true);
      }
    }
  }
  MapMeta {
    width,
    height,
    map,
  }
}

fn out_of_bounds(width: isize, height: isize, point: Point) -> bool {
  point.x < 0 || point.x >= width || point.y < 0 || point.y > height
}

fn do_coord(map: &MapMeta, view: &mut Map, origin: Point, to_check: Point) {
  // if the coordinates to check are not within bounds, return None
  if out_of_bounds(map.width, map.height, to_check) {
    return;
  }
  let offset = to_check - origin;
  let mut shadow = to_check;
  let mut blocked = false;
  while !out_of_bounds(map.width, map.height, shadow) {
    match view.get(&shadow) {
      Some(_) => match blocked {
        true => { view.remove(&shadow); },
        false => blocked = true,
      },
      None => (),
    };
    shadow += offset;
  }
}

fn calc_view(map: &MapMeta, origin: Point) -> Map {
  let mut view = map.map.clone();
  view.remove(&origin);
  let mut left = origin.x - 1;
  let mut right = origin.x + 1;
  let mut top = origin.y - 1;
  let mut bottom = origin.y + 1;
  while left >= 0 || right < map.width
    || top >= 0 || bottom < map.height {
    // top and bottom edge
    for x in left..=right {
      do_coord(&map, &mut view, origin, Point{x, y: top});
      do_coord(&map, &mut view, origin, Point{x, y: bottom});
    }
    // left and right edge
    for y in (top + 1)..bottom {
      do_coord(&map, &mut view, origin, Point{x: right, y});
      do_coord(&map, &mut view, origin, Point{x: left, y});
    }
    left -= 1;
    right += 1;
    top -= 1;
    bottom += 1;
  }
  view
}

fn find_best_roid(map: MapMeta) -> usize {
  let mut most_in_view = 0;
  for point in map.map.keys() {
    let view = calc_view(&map, *point);
    let num_in_view = view.iter().count();
    if num_in_view > most_in_view {
      most_in_view = num_in_view;
    }
  }
  most_in_view
}

fn main() -> io::Result<()> {
  let input = fs::read_to_string("input.txt")?;
  let map = string_to_map(&input);
  let best_count = find_best_roid(map);
  println!("{:?}", best_count);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  extern crate test_case;

  use test_case::test_case;

  #[test]
  fn calc_view_test() {
    let map = string_to_map(".#..#
.....
#####
....#
...##");
    let view = calc_view(&map, Point{x: 2, y: 2});
    assert_eq!(view.iter().count(), 7);
  }

  #[test_case(".#..#
.....
#####
....#
...##" => 8; "example 1")]
  #[test_case("......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####" => 33; "example 2")]
  #[test_case(".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##" => 210; "example 5")]
  fn test_find(input: &str) -> usize {
    find_best_roid(string_to_map(input))
  }
}

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::ops::Range;

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

fn display_view(view: &Map, width: isize, height: isize) {
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

fn find_best_roid(map: &MapMeta) -> Point {
  let mut most_in_view = 0;
  let mut station = Point{x: 0, y: 0};
  for point in map.map.keys() {
    let view = calc_view(&map, *point);
    let num_in_view = view.iter().count();
    if num_in_view > most_in_view {
      most_in_view = num_in_view;
      station = *point;
    }
  }
  station
}

fn smallest_offset(orig_offset: Point) -> Point {
  let x = orig_offset.x;
  let y = orig_offset.y;
  let smallest = match x.abs() < y.abs() {
    true => x.abs(),
    false => y.abs(),
  };
  if smallest < 2 {
    return orig_offset;
  }
  for div in (2..=smallest).rev() {
    if x % div == 0 && y % div == 0 {
      return Point{x: x / div, y: y / div};
    }
  }
  orig_offset
}

fn do_quadrant(map: &MapMeta, station: Point, angles: &mut Vec<Point>, x_range: Range<isize>, y_range: Range<isize>) {
  let mut quadrant: HashSet<Point> = HashSet::new();
  for x in x_range {
    for y in y_range.clone() {
      let point = Point{x, y};
      if let Some(_) = map.map.get(&point) {
        quadrant.insert(smallest_offset(Point{x, y} - station));
      }
    }
  }
  // Order them by angle
  let mut sorted: Vec<Point> = quadrant.iter().map(|x| *x).collect();
  sorted.sort_unstable_by(|a, b| {
    let a: f32 = a.y as f32 / a.x as f32;
    let b: f32 = b.y as f32 / b.x as f32;
    a.partial_cmp(&b).unwrap()
  });
  angles.append(&mut sorted);
}

fn fire_at(map: &mut MapMeta, station: Point, direction: Point) -> Option<Point> {
  let mut target = station + direction;
  while !out_of_bounds(map.width, map.height, target) {
    if let Some(_) = map.map.remove(&target) {
      return Some(target);
    }
    target += direction;
  }
  None
}

fn find_200th_roid(mut map: MapMeta, station: Point) -> Point {
  // Start pointing up
  let mut angles: Vec<Point> = vec![Point{x: 0, y: -1}];
  // Then find all points up and to the right
  do_quadrant(&map, station, &mut angles, (station.x + 1)..map.width, 0..station.y);
  // straight off to the right
  angles.push(Point{x: 1, y: 0});
  // Then find all points down and to the right
  do_quadrant(&map, station, &mut angles, (station.x + 1)..map.width, (station.y + 1)..map.height);
  // straight down
  angles.push(Point{x: 0, y: 1});
  // down and left
  do_quadrant(&map, station, &mut angles, 0..station.x, (station.y + 1)..map.height);
  // straight left
  angles.push(Point{x: -1, y: 0});
  // up and left
  do_quadrant(&map, station, &mut angles, 0..station.x, 0..station.y);
  let mut roids_to_go = 200;
  loop {
    for direction in &angles {
      if let Some(x) = fire_at(&mut map, station, *direction) {
        roids_to_go -= 1;
        if roids_to_go == 0 {
          return x;
        }
      }      
    }
  }
}

fn main() -> io::Result<()> {
  let input = fs::read_to_string("input.txt")?;
  let map = string_to_map(&input);
  let station = find_best_roid(&map);
  let roid200 = find_200th_roid(map, station);
  println!("{:?}", roid200.x * 100 + roid200.y);
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
...##" => Point{x: 3, y: 4}; "example 1")]
  #[test_case("......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####" => Point{x: 5, y: 8}; "example 2")]
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
###.##.####.##.#..##" => Point{x: 11, y: 13}; "example 5")]
  fn test_find(input: &str) -> Point {
    find_best_roid(&string_to_map(input))
  }

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
###.##.####.##.#..##" => Point{x: 8, y: 2}; "part 2")]
  fn test_fire(input: &str) -> Point {
    let map = string_to_map(input);
    let station = find_best_roid(&map);
    find_200th_roid(map, station)
  }
}

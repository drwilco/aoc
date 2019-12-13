use std::io;
use std::fs;
use std::cmp::Ordering;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, Default)]
struct Point{
  x: isize,
  y: isize,
  z: isize,
}

#[derive(Debug, Default)]
struct Vector {
  x: isize,
  y: isize,
  z: isize,
}

#[derive(Debug, Default)]
struct Moon {
  position: Point,
  velocity: Vector,
}

fn parse_num(input: &str) -> IResult<&str, isize> {
  alt((
    map(digit1, |digit_str: &str| digit_str.parse::<isize>().unwrap()),
    map(preceded(tag("-"), digit1), |digit_str: &str| 
      -1 * digit_str.parse::<isize>().unwrap()),
  ))(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
  let (input, _) = tag("<x=")(input)?;
  let (input, x) = parse_num(input)?;
  let (input, _) = tag(",")(input)?;
  let (input, _) = space0(input)?;
  let (input, _) = tag("y=")(input)?;
  let (input, y) = parse_num(input)?;
  let (input, _) = tag(",")(input)?;
  let (input, _) = space0(input)?;
  let (input, _) = tag("z=")(input)?;
  let (input, z) = parse_num(input)?;
  let (input, _) = tag(">")(input)?;
  Ok((input, Point{x, y ,z}))
}

fn apply_gravity(moons: &mut Vec<Moon>) {
  for i1 in 0..moons.len() {
    for i2 in 0..moons.len() {
      if i1 == i2 {
        continue;
      }
      match moons[i1].position.x.cmp(&moons[i2].position.x) {
        Ordering::Less => moons[i1].velocity.x += 1,
        Ordering::Greater => moons[i1].velocity.x -= 1,
        Ordering::Equal => (),
      };
      match moons[i1].position.y.cmp(&moons[i2].position.y) {
        Ordering::Less => moons[i1].velocity.y += 1,
        Ordering::Greater => moons[i1].velocity.y -= 1,
        Ordering::Equal => (),
      };
      match moons[i1].position.z.cmp(&moons[i2].position.z) {
        Ordering::Less => moons[i1].velocity.z += 1,
        Ordering::Greater => moons[i1].velocity.z -= 1,
        Ordering::Equal => (),
      };
    }
  }
}

fn apply_velocity(moons: &mut Vec<Moon>) {
  for moon in moons.iter_mut() {
    moon.position.x += moon.velocity.x;
    moon.position.y += moon.velocity.y;
    moon.position.z += moon.velocity.z;
  }
}

fn string_to_moons(string: &str) -> Vec<Moon> {
  let mut moons: Vec<Moon> = Vec::new();
  for line in string.lines() {
    let (_, position) = parse_point(line).unwrap();
    moons.push(Moon{position, ..Default::default()});
  }
  moons
}

fn calc_energy(moons: &Vec<Moon>) -> isize {
  moons.iter().map(|moon| (moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs())
                            * (moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs()) ).sum()
}

fn main() -> io::Result<()> {
  let mut moons = string_to_moons(&fs::read_to_string("input.txt")?);
  for _ in 0..1000 {
    apply_gravity(&mut moons);
    apply_velocity(&mut moons);
  }
  println!("{:?}", calc_energy(&moons));
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;
  #[test_case("<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>", 10 => 179 ; "example 1")]
  #[test_case("<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>", 100 => 1940 ; "example 2")]
  fn test(input: &str, steps: isize) -> isize {
    let mut moons = string_to_moons(input);
    for _ in 0..steps {
      apply_gravity(&mut moons);
      apply_velocity(&mut moons);
    }
    calc_energy(&moons)
  }
}
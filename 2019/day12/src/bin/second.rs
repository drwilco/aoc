use std::io;
use std::fs;
use std::cmp::Ordering;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, Default, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Point{
  x: isize,
  y: isize,
  z: isize,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Vector {
  x: isize,
  y: isize,
  z: isize,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Moon {
  position: Point,
  velocity: Vector,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Moon1D {
  position: isize,
  velocity: isize,
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

fn apply_gravity(moons: &mut Vec<Moon1D>) {
  for i1 in 0..moons.len() {
    for i2 in 0..moons.len() {
      if i1 == i2 {
        continue;
      }
      match moons[i1].position.cmp(&moons[i2].position) {
        Ordering::Less => moons[i1].velocity += 1,
        Ordering::Greater => moons[i1].velocity -= 1,
        Ordering::Equal => (),
      };
    }
  }
}

fn apply_velocity(moons: &mut Vec<Moon1D>) {
  for moon in moons.iter_mut() {
    moon.position += moon.velocity;
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

fn gcd (mut a: isize, mut b: isize) -> isize {
  while b != 0 {
    let remain = b;
    b = a % b;
    a = remain;
  }
  a
}

fn find_common_freq_gcd(input: Vec<isize>) -> isize {
  let gcd = input.iter().fold(input[0], |a, b| gcd(a, *b));
  println!("gcd: {}", gcd);
  println!("{:?}", input.iter().map(|x| *x / gcd).collect::<Vec<isize>>());
  //input.iter().map(|x| *x / gcd).product::<isize>()
  find_common_frequency(input.iter().map(|x| *x / gcd).collect::<Vec<isize>>()) * gcd
}

fn find_common_frequency(input: Vec<isize>) -> isize {
  let mut output: Vec<isize> = input.to_vec();
  let len = input.len();
  let mut max: isize;
  println!("input: {:?}", input);
  let mut counter = 0;
  while { max = *output.iter().max().unwrap(); max * (len as isize) != output.iter().sum() } {
    counter += 1;
    for i in 0..len {
      if output[i] < max {
        let diff = max - output[i];
        output[i] = max - (diff % input[i]);
        if output[i] < max {
          output[i] += input[i];
        }
      }
    }
    if counter % 1_000_000 == 0 {
      println!("current: {:?}", output);
    }
  }
  println!("took {} cycles: common: {:?}", counter, output);
  output[0]
}
/*
fn compare_and_store(cycles: &mut Option<isize>, axis: Axis, moon: isize, moons: &Vec<Moon>,
                      history: &Vec<Vec<Moon>>, cycle: isize) -> usize {
  let update: bool;
  match cycles {
    Some(_) | None => 0,
    None => {
      update = match axis {
        Axis::X => moon.position.x == initial.position.x
                    && moon.velocity.x == initial.velocity.x,
        Axis::Y => moon.position.y == initial.position.y
                    && moon.velocity.y == initial.velocity.y,
        Axis::Z => moon.position.z == initial.position.z
                    && moon.velocity.z == initial.velocity.z,
      };
      if update {
        println!("moon {:?} takes {} cycles to repeat on {:?} axis", initial.position, cycle, axis);
        *cycles = Some(cycle);
        1
      } else {
        0
      }
    }
  }
}
*/

fn find_cycle_1d(moons: &mut Vec<Moon1D>) -> isize {
  let mut cycle = 0;
//  let mut history: BTreeMap<Moon, isize> = BTreeMap::new();
  let initial = moons.to_vec();
  loop {
    apply_gravity(moons);
    apply_velocity(moons);
    cycle += 1;
    if *moons == initial {
      println!("fucking loop!");
      break;
    }
  }
  cycle
}

fn find_cycle(moons: &mut Vec<Moon>) -> isize {
  let mut x: Vec<Moon1D> = moons.iter().map(|m| Moon1D{position: m.position.x, velocity: m.velocity.x}).collect();
  let mut y: Vec<Moon1D> = moons.iter().map(|m| Moon1D{position: m.position.y, velocity: m.velocity.y}).collect();
  let mut z: Vec<Moon1D> = moons.iter().map(|m| Moon1D{position: m.position.z, velocity: m.velocity.z}).collect();
  let x_freq = find_cycle_1d(&mut x);
  let y_freq = find_cycle_1d(&mut y);
  let z_freq = find_cycle_1d(&mut z);
  find_common_frequency(vec![x_freq, y_freq, z_freq])
}

fn main() -> io::Result<()> {
  let mut moons = string_to_moons(&fs::read_to_string("input.txt")?);
  println!("{}", find_cycle(&mut moons));
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case("<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>" => 2772 ; "example 1 part 2")]
  #[test_case("<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>" => 4686774924 ; "example 2 part 2")]
  fn test_find_cycle(input: &str) -> isize {
    let mut moons = string_to_moons(input);
    find_cycle(&mut moons)
  }

  #[test_case(vec![6, 9] => 18)]
  #[test_case(vec![6, 9, 9] => 18)]
  #[test_case(vec![18, 6, 9, 9] => 18)]
  #[test_case(vec![924, 2772, 924, 2772] => 2772)]
  fn test_find_common_freq(input: Vec<isize>) -> isize {
    find_common_freq_gcd(input)
  }

  #[test_case(6, 9 => 3)]
  #[test_case(9, 6 => 3)]
  #[test_case(1071, 462 => 21)]
  fn test_gcd(a: isize, b: isize) -> isize {
    gcd(a, b)
  }
}

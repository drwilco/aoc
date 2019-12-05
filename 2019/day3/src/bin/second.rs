use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
  let f = File::open("input.txt")?;
  let f = BufReader::new(f);
  let mut lines = f.lines();
  let first_wire = lines.next().unwrap()?;
  let second_wire = lines.next().unwrap()?;

  let mut map: HashMap<i32, HashMap<i32, (i32, i32)>> = HashMap::new();

  let mut shortest_distance = 0;
  let mut x = 0;
  let mut y = 0;
  let mut length = 0;

  for op in first_wire.split(",") {
    let direction = &op[0..1];
    let mut distance: i32 = op[1..].parse().expect("must be a number");
    while distance > 0 {
      match direction {
        "U" => y += 1,
        "D" => y -= 1,
        "R" => x += 1,
        "L" => x -= 1,
        _ => panic!("invalid direction: {}", direction),
      }
      let column = map.entry(x).or_insert(HashMap::new());
      length += 1;
      column.insert(y, (length, 0));
      distance -= 1;
    }
  }

  x = 0;
  y = 0;
  length = 0;
  for op in second_wire.split(",") {
    let direction = &op[0..1];
    let mut distance: i32 = op[1..].parse().expect("must be a number");
    while distance > 0 {
      match direction {
        "U" => y += 1,
        "D" => y -= 1,
        "R" => x += 1,
        "L" => x -= 1,
        _ => panic!("invalid direction: {}", direction),
      }
      let column = map.entry(x).or_insert(HashMap::new());
      length += 1;
      let result = column.entry(y).and_modify(|(_, second)| *second = length).or_insert((0, length));
      if result.0 > 0 && result.1 > 0 {
        let from_start = result.0 + result.1;
        if shortest_distance == 0 || from_start < shortest_distance {
          shortest_distance = from_start;
        }
      }
      distance -= 1;
    }
  }

  println!("{:?}", shortest_distance);

  Ok(())
}

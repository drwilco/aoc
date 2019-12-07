use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn count_orbits(tree: &HashMap<String, String>, body: &String) -> i32 {
  match tree.get(body) {
    Some(x) => 1 + count_orbits(tree, x),
    None => 0,
  }
}

fn main() -> io::Result<()> {
  let f = File::open("input.txt")?;
  let f = BufReader::new(f);
  let mut tree: HashMap<String, String> = HashMap::new();

  for line in f.lines() {
    let line = line?;
    let mut parts = line.split(")");
    let parent = parts.next().unwrap().to_string();
    let child = parts.next().unwrap().to_string();
    tree.insert(child, parent);
  }
  let mut total = 0;
  for body in tree.keys() {
    total += count_orbits(&tree, &body);
  }
  println!("total orbits: {}", total);
  Ok(())
}

use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn get_path(tree: &HashMap<String, String>, body: &String) -> Vec<String> {
  match tree.get(body) {
    Some(x) => { let mut path = get_path(tree, x);
      path.push(body.to_string());
      path
    },
    None => vec![body.to_string()],
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
  let mut you = get_path(&tree, &"YOU".to_string());
  let mut san = get_path(&tree, &"SAN".to_string());
  while you[0] == san[0] {
    you.remove(0);
    san.remove(0);
  }
  // -2 because YOU and SAN are still on both lists
  println!("total orbit transfers: {}", you.len() + san.len() - 2);
  Ok(())
}

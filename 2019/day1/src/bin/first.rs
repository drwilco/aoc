use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
  let f = File::open("input.txt")?;
  let f = BufReader::new(f);
  let mut total: u32 = 0;

  for line in f.lines() {
    let line = line.unwrap();
    print!("in: {}", line);
    let mass: u32 = line.parse().expect("not a number");
    let fuel: u32 = mass / 3 - 2;
    println!(" out: {}", fuel);
    total += fuel;
  }
  println!("total: {}", total);
  Ok(())
}
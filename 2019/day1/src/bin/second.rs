use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn calc_fuel(mass: i32) -> i32 {
  let fuel = (mass / 3) - 2;
  if fuel <= 0 {
    return 0;
  }
  fuel + calc_fuel(fuel)
}

fn main() -> io::Result<()> {
  let f = File::open("input.txt")?;
  let f = BufReader::new(f);
  let mut total: i32 = 0;

  for line in f.lines() {
    let line = line.unwrap();
    print!("in: {}", line);
    let mass: i32 = line.parse().expect("not a number");
    let fuel: i32 = calc_fuel(mass);
    println!(" out: {}", fuel);
    total += fuel;
  }
  println!("total: {}", total);
  Ok(())
}

use std::fs;
use std::io;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() -> io::Result<()> {
  let input = fs::read_to_string("input.txt")?;
  let mut input = input.trim().chars().peekable();
  let mut lowest_zeros: i32 = std::i32::MAX;
  let mut ones = 0;
  let mut twos = 0;

  while input.peek().is_some() {
    let mut current_zeros = 0;
    let mut current_ones = 0;
    let mut current_twos = 0;
    for _ in 0..(HEIGHT * WIDTH) {
      match input.next().unwrap() {
        '0' => current_zeros += 1,
        '1' => current_ones += 1,
        '2' => current_twos += 1,
        _ => (),
      }
    }
    if current_zeros < lowest_zeros {
      lowest_zeros = current_zeros;
      ones = current_ones;
      twos = current_twos;
    } 
  }
  println!("{:?}", ones * twos);
  Ok(())
}

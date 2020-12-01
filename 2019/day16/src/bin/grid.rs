use std::io;

const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

fn generate_multiplier(inpos: usize, outpos: usize) -> isize {
  let inpos = inpos + 1; // since we always want to skip the first output
  BASE_PATTERN[(inpos / (outpos + 1) % 4) as usize]
}

fn main() -> io::Result<()> {
  for y in 0..30 {
    for x in 0..30 {
      print!("{} ", generate_multiplier(x, y));
    }
    println!();
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_generator() {
    let multipliers: [[isize; 8]; 8] = [[1, 0, -1, 0, 1, 0, -1, 0],
                                        [0, 1, 1, 0, 0, -1, -1, 0],
                                        [0, 0, 1, 1, 1, 0, 0, 0],
                                        [0, 0, 0, 1, 1, 1, 1, 0],
                                        [0, 0, 0, 0, 1, 1, 1, 1],
                                        [0, 0, 0, 0, 0, 1, 1, 1],
                                        [0, 0, 0, 0, 0, 0, 1, 1],
                                        [0, 0, 0, 0, 0, 0, 0, 1]];
    for y in 0..8 {
      for x in 0..8 {
        assert_eq!(generate_multiplier(x, y), multipliers[y][x]);
      }
    }
  }
}
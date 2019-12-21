use std::fs;
use std::io;

const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

fn generate_multiplier(inpos: usize, outpos: usize) -> isize {
  let inpos = inpos + 1; // since we always want to skip the first output
  BASE_PATTERN[(inpos / (outpos + 1) % 4) as usize]
}

fn encode(input: Vec<isize>, phases: usize) -> Vec<isize> {
  // clone, because we're going to be changing it
  let mut input = input.to_vec();
  // initialize this way, so that we can just copy output to input at the
  // top of the loop
  let mut output = input.to_vec();
  for _ in 0..phases {
    input.copy_from_slice(&output);
    for (o, outval) in output.iter_mut().enumerate() {
      *outval = input.iter().enumerate().map(|(i, x)| *x * generate_multiplier(i, o)).sum::<isize>().abs() % 10;
    }
  }
  output
}

fn main() -> io::Result<()> {
  let input = fs::read_to_string("input.txt")?
              .trim()
              .chars()
              .map(|x| x.to_digit(10).unwrap() as isize)
              .collect::<Vec<_>>();
  let output = encode(input, 100);
  println!("{}{}{}{}{}{}{}{}", output[0], output[1], output[2], output[3], output[4], output[5], output[6], output[7]);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  use test_case::test_case;

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

  #[test_case("12345678", 1 => "48226158"; "example 1")]
  #[test_case("12345678", 2 => "34040438"; "example 2")]
  #[test_case("12345678", 3 => "03415518"; "example 3")]
  #[test_case("12345678", 4 => "01029498"; "example 4")]
  #[test_case("80871224585914546619083218645595", 100 => "24176176"; "example 5")]
  #[test_case("19617804207202209144916044189917", 100 => "73745418"; "example 6")]
  #[test_case("69317163492948606335995924319873", 100 => "52432133"; "example 7")]
  fn test_part1(input: &str, cycles: usize) -> String {
    let input = input.chars()
                .map(|x| x.to_digit(10).unwrap() as isize)
                .collect::<Vec<_>>();
    let output = encode(input, cycles);
    format!("{}{}{}{}{}{}{}{}", output[0], output[1], output[2], output[3], output[4], output[5], output[6], output[7])
  }
}
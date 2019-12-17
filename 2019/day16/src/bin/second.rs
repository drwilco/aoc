use std::fs;
use std::io;
use std::cmp;
use std::time::Instant;

const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

fn generate_multiplier(inpos: usize, outpos: usize) -> isize {
  let inpos = inpos + 1; // since we always want to skip the first output
  BASE_PATTERN[(inpos / (outpos + 1) % 4) as usize]
}

fn encode(input: &Vec<isize>, phases: usize) -> Vec<isize> {
  // clone, because we're going to be changing it
  let mut input = input.to_vec();
  // initialize this way, so that we can just copy output to input at the
  // top of the loop
  let mut output = input.to_vec();
  let inlen = input.len();
  for p in 0..phases {
    let phasestart = Instant::now();
    for i in 0..inlen {
      input[i] = output[i];
    }
    for o in 0..inlen {
//      let outputstart = Instant::now();
      let mut total: isize = 0;
      let o1 = o + 1;
      for j in (o..inlen).step_by(o1) {
        match generate_multiplier(j, o) {
          1 => {
            for i in j..cmp::min(inlen, j + o1) {
              total += input[i];
            }
          },
          -1 => {
            for i in j..cmp::min(inlen, j + o1) {
              total -= input[i];
            }
          },
          _ => (),
        }
      }
      output[o] = total.abs() % 10;
//      if o % 10000 == 0 {
//        println!("{} = {} [{:?}]", o, output[o], outputstart.elapsed());
//      }
    }
    println!("phase {:?} [{:?}]", p, phasestart.elapsed());
  }
  output
}

fn repeat(input: Vec<isize>, times: usize) -> Vec<isize> {
  let mut output: Vec<isize> = Vec::with_capacity(10000 * input.len());
  for _ in 0..times {
    output.extend(&input);
  }
  output
}

fn get_message(input: Vec<isize>, output: Vec<isize>) -> String {
  let mut offset = input[0] as usize;
  for i in 1..7 {
    offset *= 10;
    offset += input[i] as usize;
  }
  let mut message = output[offset] as usize;
  for i in (offset+1)..(offset+8) {
    message *= 10;
    message += output[i] as usize;
  }
  format!("{}", message)
}

fn main() -> io::Result<()> {
  let input = fs::read_to_string("input.txt")?
              .trim()
              .chars()
              .map(|x| x.to_digit(10).unwrap() as isize)
              .collect::<Vec<_>>();
  println!("{:?}", input.len());
  let input = repeat(input, 10000);
  println!("{:?}", input.len());
  let output = encode(&input, 100);
  println!("{:?}", get_message(input, output));
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
    let output = encode(&input, cycles);
    format!("{}{}{}{}{}{}{}{}", output[0], output[1], output[2], output[3], output[4], output[5], output[6], output[7])
  }

  #[test_case("03036732577212944063491565474664", 100 => "84462026"; "example 1")]
  #[test_case("02935109699940807407585447034323", 100 => "78725270"; "example 2")]
  #[test_case("03081770884921959731165446850517", 100 => "53553731"; "example 3")]
  fn test_part2(input: &str, cycles: usize) -> String {
    let input = input.chars()
                .map(|x| x.to_digit(10).unwrap() as isize)
                .collect::<Vec<_>>();
    let input = repeat(input, 10000);
    let output = encode(&input, cycles);
    get_message(input, output)
  }
}
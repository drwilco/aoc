use std::fs;
use std::io;
use std::cmp;
use std::time::Instant;
use rayon::prelude::*;

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
    const CHUNKLEN: usize = 10_000;
    output.par_chunks_mut(CHUNKLEN).enumerate().for_each(|(chunknum, output)| {
      let chunkstart = Instant::now();
      for c in 0..output.len() {
        let o = chunknum * CHUNKLEN + c;
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
        output[c] = total.abs() % 10;
      }
      println!("chunk[{:?}] {:?}", chunknum, chunkstart.elapsed());
//      if o % 10000 == 0 {
//        println!("{} = {} [{:?}]", o, output[o], outputstart.elapsed());
//      }
    });
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

fn selective_output(mut input: Vec<isize>, needed_output: &Vec<usize>, phases: usize) -> Vec<isize> {
  let inlen = input.len();
  let mut output = Vec::with_capacity(inlen);
  output.resize(inlen, isize::default());
  const CHUNKLEN: usize = 1000;
  if phases > 1 {
    input = selective_output(input, &needed_output, phases - 1);
  }
  let phasestart = Instant::now();
  let alltemp: Vec<(usize, isize)> = needed_output.par_chunks(CHUNKLEN).flat_map(|chunk| {
    let chunktemp: Vec<(usize, isize)> = chunk.iter().map(|o| {
      let mut total: isize = 0;
      let o1 = *o + 1;
      for j in (*o..inlen).step_by(o1) {
        match generate_multiplier(j, *o) {
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
      (*o, total.abs() % 10)
    }).collect();
    chunktemp
  }).collect();
  for temp in alltemp {
    output[temp.0] = temp.1;
  }
  println!("phase {} done after {:?}", phases, phasestart.elapsed());
  output
}

fn calc_message(input: Vec<isize>) -> String {
  let mut offset = input[0] as usize;
  for i in 1..7 {
    offset *= 10;
    offset += input[i] as usize;
  }
  let mut needed_output: Vec<usize> = Vec::new();
  let inlen = input.len();
  for u in offset..inlen {
    needed_output.push(u);
  }
  let needed_output = needed_output.iter().map(|x| *x).collect();
  let output = selective_output(input, &needed_output, 100);
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
  println!("{:?}", calc_message(input));
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

  #[test_case("03036732577212944063491565474664" => "84462026"; "example 1")]
  #[test_case("02935109699940807407585447034323" => "78725270"; "example 2")]
  #[test_case("03081770884921959731165446850517" => "53553731"; "example 3")]
  fn test_part2(input: &str) -> String {
    let input = input.chars()
                .map(|x| x.to_digit(10).unwrap() as isize)
                .collect::<Vec<_>>();
    let input = repeat(input, 10000);
    calc_message(input)
  }
}

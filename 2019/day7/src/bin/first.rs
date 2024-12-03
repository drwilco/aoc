use std::io;
use std::fs;

#[derive(Debug, PartialEq)]
enum ParamMode {
  Position,
  Immediate,
}

fn param_mode(digit: isize) -> ParamMode {
  match digit {
      0 => ParamMode::Position,
      1 => ParamMode::Immediate,
      _ => panic!("invalid parameter mode: {}", digit),
  }
}

fn param_to_value(program: &Vec<isize>, pos: usize, mode: ParamMode) -> isize {
  match mode {
    ParamMode::Position => {let inpos = program[pos] as usize; program[inpos]},
    ParamMode::Immediate => program[pos],
  }
}

fn run_program(mut program: Vec<isize>, mut input: Vec<isize>) -> isize {
  let mut ip = 0; // instruction pointer
  loop {
    let mut digits = Vec::new();
    let mut remain = program[ip];
    for i in (2..5).rev() {
      digits.push(remain / 10_isize.pow(i));
      remain = remain % 10_isize.pow(i);
    }
    let opcode = remain;
    let param_3_mode = param_mode(digits[0]);
    let param_2_mode = param_mode(digits[1]);
    let param_1_mode = param_mode(digits[2]);
    match opcode {
      // in in out
      1 | 2 | 7 | 8 => {
        let inval1 = param_to_value(&program, ip + 1, param_1_mode);
        let inval2 = param_to_value(&program, ip + 2, param_2_mode);
        assert_eq!(param_3_mode, ParamMode::Position);
        let outpos = program[ip + 3] as usize;
        match opcode {
          // add
          1 => program[outpos] = inval1 + inval2,
          // multiply
          2 => program[outpos] = inval1 * inval2,
          // less than
          7 => program[outpos] = match inval1 < inval2 {
            true => 1,
            false => 0,
          },
          // equals
          8 => program[outpos] = match inval1 == inval2 {
            true => 1,
            false => 0,
          },
          _ => panic!("program error"),
        }
        ip += 4;
      },
      3 => {
        // in
        // read from input
        assert_eq!(param_1_mode, ParamMode::Position);
        let outpos = program[ip + 1] as usize;
        program[outpos] = input.remove(0);
        ip += 2;
      }
      4 => {
        // out
        // write to output
        let inval = param_to_value(&program, ip + 1, param_1_mode);
        return inval;
      }
      5 | 6 => {
        // in in
        let inval1 = param_to_value(&program, ip + 1, param_1_mode);
        let inval2 = param_to_value(&program, ip + 2, param_2_mode);
        match opcode {
          5 => {
            // jump if true
            if inval1 != 0 {
              ip = inval2 as usize;
            } else {
              ip += 3;
            }
          },
          6 => {
            // jump if false
            if inval1 == 0 {
              ip = inval2 as usize;
            } else {
              ip += 3;
            }
          },
          _ => panic!("program error"),
        }
      },
      99 => break,
      _ => panic!("invalid opcode: {}", opcode),
    }
  }
  0
}

fn run_pipe(program: &Vec<isize>, a: isize, b: isize, c: isize, d: isize, e: isize) -> isize {
  let mut output = run_program(program.to_vec(), vec![a, 0]);
  output = run_program(program.to_vec(), vec![b, output]);
  output = run_program(program.to_vec(), vec![c, output]);
  output = run_program(program.to_vec(), vec![d, output]);
  run_program(program.to_vec(), vec![e, output])
}

fn main() -> io::Result<()> {
  let program = fs::read_to_string("input.txt")?
                    .trim()
                    .split(',')
                    .map(|x| x.parse().expect("not a number"))
                    .collect::<Vec<isize>>();

  let values = vec![0, 1, 2, 3, 4];
  let mut highest_output = 0;
  for a in 0..5 {
    for b in 0..4 {
      for c in 0..3 {
        for d in 0..2 {
          let mut values = values.clone();
          let output = run_pipe(&program, values.remove(a), values.remove(b), values.remove(c), values.remove(d), values[0]);
          if output > highest_output {
            highest_output = output;
          }
        }
      }
    }
  }
  println!("highest_output: {}", highest_output);
  Ok(())
}

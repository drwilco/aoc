use std::io;
use std::fs;
use std::collections::VecDeque;

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

#[derive(Debug)]
enum ProgramState {
  Run,
  Exit,
  NeedInput,
}

fn run_program(part: &mut PipelinePart) -> ProgramState {
  loop {
    let mut digits = Vec::new();
    let mut remain = part.program[part.ip];
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
        let inval1 = param_to_value(&part.program, part.ip + 1, param_1_mode);
        let inval2 = param_to_value(&part.program, part.ip + 2, param_2_mode);
        assert_eq!(param_3_mode, ParamMode::Position);
        let outpos = part.program[part.ip + 3] as usize;
        match opcode {
          // add
          1 => part.program[outpos] = inval1 + inval2,
          // multiply
          2 => part.program[outpos] = inval1 * inval2,
          // less than
          7 => part.program[outpos] = match inval1 < inval2 {
            true => 1,
            false => 0,
          },
          // equals
          8 => part.program[outpos] = match inval1 == inval2 {
            true => 1,
            false => 0,
          },
          _ => panic!("program error"),
        }
        part.ip += 4;
      },
      3 => {
        // in
        // read from input
        if part.input_queue.len() <= 0 {
          return ProgramState::NeedInput;
        }
        assert_eq!(param_1_mode, ParamMode::Position);
        let outpos = part.program[part.ip + 1] as usize;
        part.program[outpos] = part.input_queue.pop_front().unwrap();
        part.ip += 2;
      }
      4 => {
        // out
        // write to output
        let inval = param_to_value(&part.program, part.ip + 1, param_1_mode);
        part.output_queue.push_back(inval);
        part.ip += 2;
      }
      5 | 6 => {
        // in in
        let inval1 = param_to_value(&part.program, part.ip + 1, param_1_mode);
        let inval2 = param_to_value(&part.program, part.ip + 2, param_2_mode);
        match opcode {
          5 => {
            // jump if true
            if inval1 != 0 {
              part.ip = inval2 as usize;
            } else {
              part.ip += 3;
            }
          },
          6 => {
            // jump if false
            if inval1 == 0 {
              part.ip = inval2 as usize;
            } else {
              part.ip += 3;
            }
          },
          _ => panic!("program error"),
        }
      },
      99 => break,
      _ => panic!("invalid opcode: {}", opcode),
    }
  }
  ProgramState::Exit
}

#[derive(Debug)]
struct PipelinePart {
  program: Vec<isize>,
  state: ProgramState,
  input_queue: VecDeque<isize>,
  output_queue: VecDeque<isize>,
  ip: usize,
  previous: usize,
}

fn run_pipe(program: &Vec<isize>, a: isize, b: isize, c: isize, d: isize, e: isize) -> isize {
  let mut pipeline: [PipelinePart; 5] = [
    PipelinePart{
      program: program.to_vec(),
      state: ProgramState::Run,
      input_queue: VecDeque::from(vec![a, 0]),
      output_queue: VecDeque::new(),
      ip: 0,
      previous: 4},
    PipelinePart{
      program: program.to_vec(),
      state: ProgramState::Run,
      input_queue: VecDeque::from(vec![b]),
      output_queue: VecDeque::new(),
      ip: 0,
      previous: 0},
    PipelinePart{
      program: program.to_vec(),
      state: ProgramState::Run,
      input_queue: VecDeque::from(vec![c]),
      output_queue: VecDeque::new(),
      ip: 0,
      previous: 1},
    PipelinePart{
      program: program.to_vec(),
      state: ProgramState::Run,
      input_queue: VecDeque::from(vec![d]),
      output_queue: VecDeque::new(),
      ip: 0,
      previous: 2},
    PipelinePart{
      program: program.to_vec(),
      state: ProgramState::Run,
      input_queue: VecDeque::from(vec![e]),
      output_queue: VecDeque::new(),
      ip: 0,
      previous: 3},
  ];
  let mut exited = 0;
  while exited < 5 {
    exited = 0;
    for index in 0..5 {
      loop {
        match pipeline[index].state {
          ProgramState::Exit => {
            exited += 1;
            break; // go to the next part
          },
          ProgramState::Run => {
            pipeline[index].state = run_program(&mut pipeline[index]);
          },
          ProgramState::NeedInput => {
            if pipeline[pipeline[index].previous].output_queue.len() > 0 {
              let temp = pipeline[pipeline[index].previous].output_queue.pop_front().unwrap();
              pipeline[index].input_queue.push_back(temp);
              pipeline[index].state = ProgramState::Run;
            } else {
              match pipeline[pipeline[index].previous].state {
                ProgramState::Exit => panic!("pipeline fucked: {:?}", pipeline),
                _ => break, // go to the next part
              }
            }
          }
        }
      }
    }
  }
  pipeline[4].output_queue.pop_front().unwrap()
}

fn main() -> io::Result<()> {
  let program = fs::read_to_string("input.txt")?
                    .trim()
                    .split(',')
                    .map(|x| x.parse().expect("not a number"))
                    .collect::<Vec<isize>>();

  let values = vec![5, 6, 7, 8, 9];
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
    let output = run_pipe(&program, 9, 8, 7, 6, 5);
    assert_eq!(output, 139629729);
  }

  #[test]
  fn test_2() {
    let program = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
    let output = run_pipe(&program, 9, 7, 8, 5, 6);
    assert_eq!(output, 18216);
  }
}

use std::io;
use std::fs;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum ParamMode {
  Position,
  Immediate,
  Relative,
}

fn param_mode(digit: isize) -> ParamMode {
  match digit {
      0 => ParamMode::Position,
      1 => ParamMode::Immediate,
      2 => ParamMode::Relative,
      _ => panic!("invalid parameter mode: {}", digit),
  }
}

fn param_to_position(program: &mut Vec<isize>, param_pos: usize, mode: ParamMode, rel_base: isize) -> usize {
  let result = match mode {
    ParamMode::Position => program[param_pos] as usize,
    ParamMode::Relative => (program[param_pos] + rel_base) as usize,
    ParamMode::Immediate => panic!("Immediate can not be a position"),
  };
  if result >= program.len() {
    program.resize(result + 1, 0);
  }
  result
}

fn param_to_value(program: &mut Vec<isize>, param_pos: usize, mode: ParamMode, rel_base: isize) -> isize {
  match mode {
    ParamMode::Position | ParamMode::Relative => {
      let abs_pos = param_to_position(program, param_pos, mode, rel_base);
      program[abs_pos]
    },
    ParamMode::Immediate => program[param_pos],
  }
}

#[derive(Debug)]
enum ProgramState {
  Run,
  Exit,
  NeedInput,
}

#[derive(Debug)]
enum OpCode {
  Add = 1,
  Multiply = 2,
  Read = 3,
  Write = 4,
  JumpIfTrue = 5,
  JumpIfFalse = 6,
  LessThan = 7,
  EqualTo = 8,
  AdjustBase = 9,
  Exit = 99,
}
use OpCode::*;

fn in_in_out(opcode: OpCode, inval1: isize, inval2: isize) -> isize {
  match opcode {
    Add => inval1 + inval2,
    Multiply => inval1 * inval2,
    LessThan => match inval1 < inval2 {
      true => 1,
      false => 0,
    },
    EqualTo => match inval1 == inval2 {
      true => 1,
      false => 0,
    },
    _ => panic!("program error"),
  }
}

fn in_in_jump(opcode: OpCode, inval1: isize, inval2: isize, ip: &mut usize) {
  match opcode {
    JumpIfTrue => {
      // jump if true
      if inval1 != 0 {
        *ip = inval2 as usize;
      } else {
        *ip += 3;
      }
    },
    JumpIfFalse => {
      // jump if false
      if inval1 == 0 {
        *ip = inval2 as usize;
      } else {
        *ip += 3;
      }
    },
    _ => panic!("program error"),
  }  
}

fn run_program(part: &mut PipelinePart) -> ProgramState {
  let program = &mut part.program;
  let ip = &mut part.ip;
  let rel_base = &mut part.rel_base;
  loop {
    let mut digits = Vec::new();
    let mut remain = program[*ip];
    for i in (2..5).rev() {
      digits.push(remain / 10_isize.pow(i));
      remain = remain % 10_isize.pow(i);
    }
    let opcode = match remain {
      1 => Add,
      2 => Multiply,
      3 => Read,
      4 => Write,
      5 => JumpIfTrue,
      6 => JumpIfFalse,
      7 => LessThan,
      8 => EqualTo,
      9 => AdjustBase,
      99 => Exit,
      _ => panic!("invalid opcode: {}", remain),
    };
    let param_3_mode = param_mode(digits[0]);
    let param_2_mode = param_mode(digits[1]);
    let param_1_mode = param_mode(digits[2]);
    match opcode {
      // in in out
      Add | Multiply | LessThan | EqualTo => {
        let inval1 = param_to_value(program, *ip + 1, param_1_mode, *rel_base);
        let inval2 = param_to_value(program, *ip + 2, param_2_mode, *rel_base);
        let outpos = param_to_position(program, *ip + 3, param_3_mode, *rel_base);
        program[outpos] = in_in_out(opcode, inval1, inval2);
        *ip += 4;
      },
      // out
      Read => {
        // read from input
        let outpos = param_to_position(program, *ip + 1, param_1_mode, *rel_base);
        if part.input_queue.len() <= 0 {
          return ProgramState::NeedInput;
        }
        program[outpos] = part.input_queue.pop_front().unwrap();
        *ip += 2;
      },
      // in
      Write | AdjustBase => {
        let inval = param_to_value(program, *ip + 1, param_1_mode, *rel_base);
        match opcode {
          Write => part.output_queue.push_back(inval),
          AdjustBase => *rel_base += inval,
          _ => panic!("program error"),          
        }
        *ip += 2;
      },
      // in in
      JumpIfTrue | JumpIfFalse => {
        let inval1 = param_to_value(program, *ip + 1, param_1_mode, *rel_base);
        let inval2 = param_to_value(program, *ip + 2, param_2_mode, *rel_base);
        in_in_jump(opcode, inval1, inval2, ip);
      },
      Exit => break,
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
  rel_base: isize,
}

fn run_pipe(pipeline: &mut Vec<PipelinePart>) -> io::Result<()> {
  let mut exited = 0;
  while exited < pipeline.len() {
    exited = 0;
    for index in 0..pipeline.len() {
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
            let previdx = pipeline[index].previous;
            if previdx == std::usize::MAX {
              println!("please provide input: ");
              let mut buf = String::new();
              io::stdin().read_line(&mut buf).expect("input error");
              let temp = buf.trim().parse().expect("not a number");
              pipeline[index].input_queue.push_back(temp);
            } else if pipeline[previdx].output_queue.len() > 0 {
              let temp = pipeline[previdx].output_queue.pop_front().unwrap();
              pipeline[index].input_queue.push_back(temp);
              pipeline[index].state = ProgramState::Run;
            } else {
              match pipeline[previdx].state {
                ProgramState::Exit => panic!("pipeline fucked: {:?}", pipeline),
                _ => break, // go to the next part
              }
            }
          }
        }
      }
    }
  }
  Ok(())
}

fn main() -> io::Result<()> {
  let program = fs::read_to_string("input.txt")?
                    .trim()
                    .split(',')
                    .map(|x| x.parse().expect("not a number"))
                    .collect::<Vec<isize>>();
  let mut pipeline = vec![
    PipelinePart{
      program: program.to_vec(),
      state: ProgramState::Run,
      input_queue: VecDeque::from(vec![2]),
      output_queue: VecDeque::new(),
      ip: 0,
      previous: std::usize::MAX,
      rel_base: 0,
    },
  ];
  run_pipe(&mut pipeline).expect("something went wrong");
  println!("{:?}", pipeline[0].output_queue);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    let mut pipeline = vec![
      PipelinePart{
        program: program.to_vec(),
        state: ProgramState::Run,
        input_queue: VecDeque::new(),
        output_queue: VecDeque::new(),
        ip: 0,
        previous: std::usize::MAX,
        rel_base: 0,
      },
    ];
    run_pipe(&mut pipeline).expect("something went wrong");
    let output = pipeline[0].output_queue.iter().map(|x| *x).collect::<Vec<isize>>();
    assert_eq!(program, output);
  }

  #[test]
  fn test_2() {
    let program = vec![1102,34915192,34915192,7,4,7,99,0];
    let mut pipeline = vec![
      PipelinePart{
        program: program.to_vec(),
        state: ProgramState::Run,
        input_queue: VecDeque::new(),
        output_queue: VecDeque::new(),
        ip: 0,
        previous: std::usize::MAX,
        rel_base: 0,
      },
    ];
    run_pipe(&mut pipeline).expect("something went wrong");
    let output = pipeline[0].output_queue.pop_front().unwrap();
    assert_eq!(format!("{}", output).len(), 16);
  }

  #[test]
  fn test_3() {
    let program = vec![104,1125899906842624,99];
    let mut pipeline = vec![
      PipelinePart{
        program: program.to_vec(),
        state: ProgramState::Run,
        input_queue: VecDeque::new(),
        output_queue: VecDeque::new(),
        ip: 0,
        previous: std::usize::MAX,
        rel_base: 0,
      },
    ];
    run_pipe(&mut pipeline).expect("something went wrong");
    let output = pipeline[0].output_queue.pop_front().unwrap();
    assert_eq!(output, 1125899906842624);
  }
}

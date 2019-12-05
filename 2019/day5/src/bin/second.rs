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

fn main() -> io::Result<()> {
  let mut program = fs::read_to_string("input.txt")?
                    .trim()
                    .split(',')
                    .map(|x| x.parse().expect("not a number"))
                    .collect::<Vec<isize>>();

  println!("program: {:?}", program);
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
      1 | 2 | 7 | 8 => {
        let inval1 = param_to_value(&program, ip + 1, param_1_mode);
        let inval2 = param_to_value(&program, ip + 2, param_2_mode);
        assert_eq!(param_3_mode, ParamMode::Position);
        let outpos = program[ip + 3] as usize;
        match opcode {
          1 => program[outpos] = inval1 + inval2,
          2 => program[outpos] = inval1 * inval2,
          7 => program[outpos] = match inval1 < inval2 {
            true => 1,
            false => 0,
          },
          8 => program[outpos] = match inval1 == inval2 {
            true => 1,
            false => 0,
          },
          _ => panic!("program error"),
        }
        ip += 4;
      },
      3 => {
        assert_eq!(param_1_mode, ParamMode::Position);
        let outpos = program[ip + 1] as usize;
        println!("please provide input: ");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("input error");
        program[outpos] = buf.trim().parse().expect("not a number");
        ip += 2;
      }
      4 => {
        let inval = param_to_value(&program, ip + 1, param_1_mode);
        println!("{}", inval);
        ip += 2;
      }
      5 | 6 => {
        let inval1 = param_to_value(&program, ip + 1, param_1_mode);
        let inval2 = param_to_value(&program, ip + 2, param_2_mode);
        match opcode {
          5 => {
            if inval1 != 0 {
              ip = inval2 as usize;
            } else {
              ip += 3;
            }
          },
          6 => {
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
  Ok(())
}

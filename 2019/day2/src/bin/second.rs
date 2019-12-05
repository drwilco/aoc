use std::io;
use std::fs;

fn main() -> io::Result<()> {
  let original_program = fs::read_to_string("input.txt")?
                          .trim()
                          .split(',')
                          .map(|x| x.parse().expect("not a number"))
                          .collect::<Vec<usize>>();
  for noun in 0..100 {
    for verb in 0..100 {
      let mut program = original_program.to_vec();
      program[1] = noun;
      program[2] = verb;
      if run_program(program) == 19690720 {
        println!("Answer: {}", 100*noun + verb);
        std::process::exit(0);
      }
    }
  }
  Ok(())
}

fn run_program(mut program: Vec<usize>) -> usize {
  let mut cursor = 0;
  loop {
//    println!("{}", program.iter()
//                          .enumerate()
//                          .fold(String::new(),
//                                |acc, (index, num)|
//                                  acc + match index == cursor {true => "*", false => ""} + &index.to_string() + ":" + &num.to_string() + ", "
//                          )
//    );
    match program[cursor] {
      1 | 2 => {
        let in1 = program[cursor + 1];
        let in2 = program[cursor + 2];
        let out = program[cursor + 3];
        match program[cursor] {
          1 => program[out] = program[in1] + program[in2],
          2 => program[out] = program[in1] * program[in2],
          _ => panic!("program error"),
        }
        cursor += 4;
      },
      99 => break,
      _ => panic!("program error"),
    }
  }
  program[0]
}
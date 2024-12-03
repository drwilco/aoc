use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::io;
use std::ops;

#[derive(Debug, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PartState {
    Run,
    Exit,
    NeedInput,
}

impl Default for PartState {
    fn default() -> Self {
        PartState::Run
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

impl ops::Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
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

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Default)]
struct Point {
    x: isize,
    y: isize,
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Point {
        match rhs {
            Direction::North => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::East => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl ops::AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::North => self.y -= 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
            Direction::East => self.x += 1,
        };
    }
}

trait PipelinePart {
    fn get_input_queue(&self) -> VecDeque<isize>;
    fn get_output_queue(&self) -> VecDeque<isize>;
    fn get_state(&self) -> PartState;
    fn run(&mut self);
}

impl fmt::Debug for dyn PipelinePart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "state={:?} input={:?} output={:?})",
            self.get_state(),
            self.get_input_queue(),
            self.get_output_queue()
        )
    }
}

#[derive(Debug, Default, Clone)]
struct IntCode {
    input_queue: VecDeque<isize>,
    output_queue: VecDeque<isize>,
    state: PartState,
    program: Vec<isize>,
    ip: usize,
    rel_base: isize,
}

impl PipelinePart for IntCode {
    fn get_input_queue(&self) -> VecDeque<isize> {
        self.input_queue.clone()
    }
    fn get_output_queue(&self) -> VecDeque<isize> {
        self.output_queue.clone()
    }
    fn get_state(&self) -> PartState {
        self.state
    }
    fn run(&mut self) {
        self.state = run_program(self);
    }
}

fn param_mode(digit: isize) -> ParamMode {
    match digit {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        2 => ParamMode::Relative,
        _ => panic!("invalid parameter mode: {}", digit),
    }
}

fn param_to_position(
    program: &mut Vec<isize>,
    param_pos: usize,
    mode: ParamMode,
    rel_base: isize,
) -> usize {
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

fn param_to_value(
    program: &mut Vec<isize>,
    param_pos: usize,
    mode: ParamMode,
    rel_base: isize,
) -> isize {
    match mode {
        ParamMode::Position | ParamMode::Relative => {
            let abs_pos = param_to_position(program, param_pos, mode, rel_base);
            program[abs_pos]
        }
        ParamMode::Immediate => program[param_pos],
    }
}

fn in_in_out(opcode: OpCode, inval1: isize, inval2: isize) -> isize {
    match opcode {
        Add => inval1 + inval2,
        Multiply => inval1 * inval2,
        LessThan => {
            if inval1 < inval2 {
                1
            } else {
                0
            }
        }
        EqualTo => {
            if inval1 == inval2 {
                1
            } else {
                0
            }
        }
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
        }
        JumpIfFalse => {
            // jump if false
            if inval1 == 0 {
                *ip = inval2 as usize;
            } else {
                *ip += 3;
            }
        }
        _ => panic!("program error"),
    }
}

fn run_program(intcode: &mut IntCode) -> PartState {
    let program = &mut intcode.program;
    let ip = &mut intcode.ip;
    let rel_base = &mut intcode.rel_base;
    loop {
        let mut digits = Vec::new();
        let mut remain = program[*ip];
        for i in (2..5).rev() {
            digits.push(remain / 10_isize.pow(i));
            remain %= 10_isize.pow(i);
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
            }
            // out
            Read => {
                // read from input
                let outpos = param_to_position(program, *ip + 1, param_1_mode, *rel_base);
                if intcode.input_queue.is_empty() {
                    return PartState::NeedInput;
                }
                program[outpos] = intcode.input_queue.pop_front().unwrap();
                *ip += 2;
            }
            // in
            Write | AdjustBase => {
                let inval = param_to_value(program, *ip + 1, param_1_mode, *rel_base);
                match opcode {
                    Write => {
                        intcode.output_queue.push_back(inval);
                        *ip += 2;
                        return PartState::Run;
                    }
                    AdjustBase => *rel_base += inval,
                    _ => panic!("program error"),
                }
                *ip += 2;
            }
            // in in
            JumpIfTrue | JumpIfFalse => {
                let inval1 = param_to_value(program, *ip + 1, param_1_mode, *rel_base);
                let inval2 = param_to_value(program, *ip + 2, param_2_mode, *rel_base);
                in_in_jump(opcode, inval1, inval2, ip);
            }
            Exit => break,
        }
    }
    PartState::Exit
}

fn main() -> io::Result<()> {
    let program = fs::read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse().expect("not a number"))
        .collect::<Vec<isize>>();
    let clean_intcode = IntCode {
        program,
        ..Default::default()
    };
    let mut count = 0;
    for x in 0..50 {
        for y in 0..50 {
            let mut intcode = clean_intcode.clone();
            intcode.input_queue.push_back(x);
            intcode.input_queue.push_back(y);
            intcode.run();
            match intcode.output_queue.pop_front() {
                Some(0) => {
                    print!(".");
                },
                Some(1) => {
                    count += 1;
                    print!("#");
                }
                Some(x) => panic!("unexpected output from program: {}", x),
                None => panic!("no output from program"),
            }
        }
        println!();
    }
    println!("{:?}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test]
    fn day9_test_1() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut intcode = IntCode {
            program: program.to_vec(),
            ..Default::default()
        };
        while intcode.get_state() == PartState::Run {
            intcode.run();
        }
        let output = intcode
            .output_queue
            .iter()
            .map(|x| *x)
            .collect::<Vec<isize>>();
        assert_eq!(program, output);
    }

    #[test_case( vec![1,9,10,3,2,3,11,0,99,30,40,50] => vec![3500,9,10,70,2,3,11,0,99,30,40,50] ; "day 2 example 1")]
    #[test_case( vec![1,0,0,0,99] => vec![2,0,0,0,99] ; "day 2 example 2")]
    #[test_case( vec![2,3,0,3,99] => vec![2,3,0,6,99] ; "day 2 example 3")]
    #[test_case( vec![2,4,4,5,99,0] => vec![2,4,4,5,99,9801] ; "day 2 example 4")]
    #[test_case( vec![1,1,1,4,99,5,6,0,99] => vec![30,1,1,4,2,5,6,0,99] ; "day 2 example 5")]
    #[test_case( vec![1002,4,3,4,33] => vec![1002,4,3,4,99] ; "day 5 example 1")]
    #[test_case( vec![1101,100,-1,4,0] => vec![1101,100,-1,4,99] ; "day 5 example 2")]
    fn pre_input_output(program: Vec<isize>) -> Vec<isize> {
        let mut intcode = IntCode {
            program: program.to_vec(),
            ..Default::default()
        };
        intcode.run();
        intcode.program.to_vec()
    }
}

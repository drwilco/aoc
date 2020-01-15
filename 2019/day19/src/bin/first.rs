use std::char;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::io;
use std::ops;
use std::process;
use std::thread;
use std::time::Duration;

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

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Right,
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

impl ops::AddAssign<Turn> for Direction {
    fn add_assign(&mut self, rhs: Turn) {
        *self = match rhs {
            Turn::Left => match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::East => Direction::North,
            },
            Turn::Right => match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::East => Direction::South,
            },
        };
    }
}

impl ops::Add<Turn> for Direction {
    type Output = Direction;

    fn add(self, rhs: Turn) -> Direction {
        match rhs {
            Turn::Left => match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::East => Direction::North,
            },
            Turn::Right => match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::East => Direction::South,
            },
        }
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
    fn push_input(&mut self, input: isize);
    fn pop_output(&mut self) -> isize;
    fn output_available(&self) -> bool;
    fn get_state(&self) -> PartState;
    fn get_previous(&self) -> Option<usize>;
    fn run(&mut self);
    fn broken_pipe(&mut self);
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
    previous: Option<usize>,
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
    fn push_input(&mut self, input: isize) {
        self.input_queue.push_back(input);
    }
    fn pop_output(&mut self) -> isize {
        self.output_queue.pop_front().unwrap()
    }
    fn output_available(&self) -> bool {
        !self.output_queue.is_empty()
    }
    fn get_state(&self) -> PartState {
        self.state
    }
    fn get_previous(&self) -> Option<usize> {
        self.previous
    }
    fn run(&mut self) {
        self.state = run_program(self);
    }
    fn broken_pipe(&mut self) {
        panic!("IntCode expects input, but provider exited");
    }
}

#[derive(Debug, PartialEq, Clone)]
enum TileType {
    Scaffold,
    Open,
    Intersection,
}

type TileMap = HashMap<Point, TileType>;

#[derive(Debug, PartialEq)]
enum DroneState {
    Init,
    GetResult,
}

impl Default for DroneState {
    fn default() -> Self {
        DroneState::Init
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

fn run_pipe(pipeline: &mut Vec<Box<dyn PipelinePart>>) -> io::Result<()> {
    let mut exited = 0;
    while exited < pipeline.len() {
        exited = 0;
        for index in 0..pipeline.len() {
            loop {
                match pipeline[index].get_state() {
                    PartState::Exit => {
                        exited += 1;
                        break; // go to the next part
                    }
                    PartState::Run => {
                        pipeline[index].run();
                        if pipeline[index].output_available() {
                            break;
                        }
                    }
                    PartState::NeedInput => {
                        match pipeline[index].get_previous() {
                            None => {
                                println!("please provide input: ");
                                let mut buf = String::new();
                                io::stdin().read_line(&mut buf).expect("input error");
                                let temp = match buf.trim().parse() {
                                    Ok(x) => x,
                                    Err(_) => break,
                                };
                                pipeline[index].push_input(temp);
                                pipeline[index].run();
                            }
                            Some(previdx) => {
                                if pipeline[previdx].output_available() {
                                    let temp = pipeline[previdx].pop_output();
                                    pipeline[index].push_input(temp);
                                    pipeline[index].run();
                                } else {
                                    match pipeline[previdx].get_state() {
                                        PartState::Exit => pipeline[index].broken_pipe(),
                                        _ => break, // go to the next part
                                    }
                                }
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

    #[test]
    fn day9_test_2() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut intcode = IntCode {
            program: program.to_vec(),
            ..Default::default()
        };
        intcode.run();
        assert_eq!(format!("{}", intcode.pop_output()).len(), 16);
    }

    #[test]
    fn day9_test_3() {
        let program = vec![104, 1125899906842624, 99];
        let mut intcode = IntCode {
            program: program.to_vec(),
            ..Default::default()
        };
        intcode.run();
        assert_eq!(intcode.pop_output(), 1125899906842624);
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

    #[test_case( vec![3,9,8,9,10,9,4,9,99,-1,8], 8 => 1 ; "day 5 example 3a - equal to position mode")]
    #[test_case( vec![3,9,8,9,10,9,4,9,99,-1,8], 234 => 0 ; "day 5 example 3b - equal to position mode")]
    #[test_case( vec![3,9,7,9,10,9,4,9,99,-1,8], 7 => 1 ; "day 5 example 4a - less than position mode")]
    #[test_case( vec![3,9,7,9,10,9,4,9,99,-1,8], 8 => 0 ; "day 5 example 4b - less than position mode")]
    #[test_case( vec![3,9,7,9,10,9,4,9,99,-1,8], 9 => 0 ; "day 5 example 4c - less than position mode")]
    #[test_case( vec![3,9,7,9,10,9,4,9,99,-1,8], -9 => 1 ; "day 5 example 4d - less than position mode")]
    #[test_case( vec![3,3,1108,-1,8,3,4,3,99], 8 => 1 ; "day 5 example 5a - equal to immediate mode")]
    #[test_case( vec![3,3,1108,-1,8,3,4,3,99], 234 => 0 ; "day 5 example 5b - equal to immediate mode")]
    #[test_case( vec![3,3,1107,-1,8,3,4,3,99], 7 => 1 ; "day 5 example 6a - less than immediate mode")]
    #[test_case( vec![3,3,1107,-1,8,3,4,3,99], 8 => 0 ; "day 5 example 6b - less than immediate mode")]
    #[test_case( vec![3,3,1107,-1,8,3,4,3,99], 9 => 0 ; "day 5 example 6c - less than immediate mode")]
    #[test_case( vec![3,3,1107,-1,8,3,4,3,99], -9 => 1 ; "day 5 example 6d - less than immediate mode")]
    #[test_case( vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0 => 0 ; "day 5 example 7a - jump position mode")]
    #[test_case( vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 1 => 1 ; "day 5 example 7b - jump position mode")]
    #[test_case( vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], -99 => 1 ; "day 5 example 7c - jump position mode")]
    #[test_case( vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 0 => 0 ; "day 5 example 8a - jump immediate mode")]
    #[test_case( vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 1 => 1 ; "day 5 example 8b - jump immediate mode")]
    #[test_case( vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], -99 => 1 ; "day 5 example 8c - jump immediate mode")]
    #[test_case( vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104
    ,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], -99 => 999 ; "day 5 example 9a - long example")]
    #[test_case( vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104
    ,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 8 => 1000 ; "day 5 example 9b - long example")]
    #[test_case( vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104
    ,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 1337 => 1001 ; "day 5 example 9c - long example")]

    fn simple_input_output(program: Vec<isize>, input: isize) -> isize {
        let mut pipeline: Vec<Box<dyn PipelinePart>> = vec![Box::new(IntCode {
            program: program.to_vec(),
            input_queue: VecDeque::from(vec![input]),
            ..Default::default()
        })];
        run_pipe(&mut pipeline).expect("something went wrong");
        pipeline[0].pop_output()
    }

    #[test_case( vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], vec![4,3,2,1,0], 0 => 43210 ; "day 7 example 1")]
    #[test_case( vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], vec![0,1,2,3,4],
    0 => 54321 ; "day 7 example 2")]
    #[test_case( vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0],
    vec![1,0,4,3,2], 0 => 65210 ; "day 7 example 3")]
    fn amp_chain(program: Vec<isize>, settings: Vec<isize>, input: isize) -> isize {
        let mut pipeline: Vec<Box<dyn PipelinePart>> = Vec::new();
        for (i, s) in settings.iter().enumerate() {
            pipeline.push(Box::new(IntCode {
                program: program.to_vec(),
                input_queue: VecDeque::from(vec![*s]),
                previous: match i {
                    0 => None,
                    _ => Some(i - 1),
                },
                ..Default::default()
            }));
        }
        pipeline[0].push_input(input);
        run_pipe(&mut pipeline).expect("something went wrong");
        pipeline[settings.len() - 1].pop_output()
    }

    #[test_case( vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5],
    vec![9,8,7,6,5], 0 => 139629729 ; "day 7 example 4")]
    #[test_case( vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54
    ,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10], vec![9,7,8,5,6], 0 => 18216 ; "day 7 example 5")]
    fn amp_chain_feedback(program: Vec<isize>, settings: Vec<isize>, input: isize) -> isize {
        let mut pipeline: Vec<Box<dyn PipelinePart>> = Vec::new();
        for (i, s) in settings.iter().enumerate() {
            pipeline.push(Box::new(IntCode {
                program: program.to_vec(),
                input_queue: VecDeque::from(vec![*s]),
                previous: match i {
                    0 => Some(settings.len() - 1),
                    _ => Some(i - 1),
                },
                ..Default::default()
            }));
        }
        pipeline[0].push_input(input);
        run_pipe(&mut pipeline).expect("something went wrong");
        pipeline[settings.len() - 1].pop_output()
    }
}

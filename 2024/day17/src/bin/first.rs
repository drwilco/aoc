#![feature(test)]

use nom::{
    bytes::complete::tag,
    character::complete::{char as nom_char, line_ending, u64 as nom_u64, u8},
    multi::separated_list1,
    IResult,
};
use std::fs;

#[derive(Clone, Copy, Debug)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

fn parse_input(input: &str) -> IResult<&str, (Registers, Vec<u8>)> {
    let (input, _) = tag("Register A: ")(input)?;
    let (input, register_a) = nom_u64(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = tag("Register B: ")(input)?;
    let (input, register_b) = nom_u64(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = tag("Register C: ")(input)?;
    let (input, register_c) = nom_u64(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = tag("Program: ")(input)?;
    let (input, program) = separated_list1(nom_char(','), u8)(input)?;
    let (input, _) = line_ending(input)?;
    assert_eq!(input, "");
    Ok((
        input,
        (
            Registers {
                a: register_a,
                b: register_b,
                c: register_c,
            },
            program,
        ),
    ))
}

fn get_combo(operand: u8, registers: &Registers) -> u64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        7 => panic!("Invalid program (combo operand 7)"),
        _ => panic!("Invalid program (combo operand != u3)"),
    }
}

fn run_program(registers: &mut Registers, program: &[u8]) -> Vec<u8> {
    let mut ip = 0;
    let mut output = Vec::new();
    while ip < program.len() {
        let [opcode, operand] = program[ip..ip + 2] else {
            panic!("Invalid program")
        };
        match opcode {
            // adv
            0 => {
                registers.a /= 2_u64.pow(u32::try_from(get_combo(operand, registers)).unwrap());
            }
            // bxl
            1 => {
                registers.b ^= u64::from(operand);
            }
            // bst
            2 => {
                registers.b = get_combo(operand, registers) % 8;
            }
            // jnz
            3 => {
                if registers.a != 0 {
                    ip = operand as usize;
                } else {
                    ip += 2;
                }
            }
            // bxc
            4 => {
                registers.b ^= registers.c;
            }
            // out
            5 => {
                let out = (get_combo(operand, registers) % 8) as u8;
                output.push(out);
            }
            // bdv
            6 => {
                registers.b =
                    registers.a / 2_u64.pow(u32::try_from(get_combo(operand, registers)).unwrap());
                }
            // cdv
            7 => {
                registers.c =
                    registers.a / 2_u64.pow(u32::try_from(get_combo(operand, registers)).unwrap());
                }
            _ => panic!("Invalid program (instruction != u3)"),
        }
        if opcode != 3 {
            ip += 2;
        }
    }
    output
}

fn solve(registers: &mut Registers, program: &[u8]) -> String {
    let output = run_program(registers, program);
    output.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn run(input: &str) -> String {
    let (input, (mut registers, program)) = parse_input(input).unwrap();
    assert_eq!(input, "");
    solve(&mut registers, &program)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    extern crate test as std_test;
    use super::*;
    use std_test::{black_box, Bencher};
    use test_case::test_case;

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = black_box(&input);
        b.iter(|| parse_input(input));
    }

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let (input, (registers, program)) = parse_input(&input).unwrap();
        assert_eq!(input, "");
        let registers = black_box(&registers);
        let program = black_box(&program);
        b.iter(|| {
            let mut registers = *registers;
            solve(&mut registers, program)
        });
    }

    #[test_case("Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
" => "4,6,3,5,6,3,5,2,1,0"; "full example")]
    fn test(input: &str) -> String {
        run(input)
    }

    #[test_case(0, 0, 9, &[2,6] => (0, 1, 9, vec![]); "example 1")]
    #[test_case(10, 0, 0, &[5,0,5,1,5,4] => (10, 0, 0, vec![0, 1, 2]); "example 2")]
    #[test_case(2024, 0, 0, &[0,1,5,4,3,0] => (0, 0, 0, vec![4,2,5,6,7,7,7,7,3,1,0]); "example 3")]
    #[test_case(0, 29, 0, &[1,7] => (0, 26, 0, vec![]); "example 4")]
    #[test_case(0, 2024, 43690, &[4,0] => (0, 44354, 43690, vec![]); "example 5")]
    #[test_case(2024, 0, 0, &[6,2] => (2024, 506, 0, vec![]); "test bdv")]
    #[test_case(2024, 0, 0, &[7,3] => (2024, 0, 253, vec![]); "test cdv")]
    fn test_run_program(
        reg_a: u64,
        reg_b: u64,
        reg_c: u64,
        program: &[u8],
    ) -> (u64, u64, u64, Vec<u8>) {
        let mut registers = Registers {
            a: reg_a,
            b: reg_b,
            c: reg_c,
        };
        let output = run_program(&mut registers, program);
        (registers.a, registers.b, registers.c, output)
    }
}

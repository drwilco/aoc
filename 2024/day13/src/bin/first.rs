#![feature(test)]

use std::{cmp::min, fs};

use nom::{branch::alt, bytes::complete::tag, character::complete::{char, line_ending, u32}, combinator::{eof, value}, multi::separated_list1, sequence::terminated, IResult};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Button {
    cost: u32,
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct ClawMachine {
    buttons: [Button; 2],
    x: u32,
    y: u32,
}

fn parse_button(input: &str) -> IResult<&str, Button> {
    let (input, _) = tag("Button ")(input)?;
    let (input, cost) = alt((value(3, char('A')), value(1, char('B'))))(input)?;
    let (input, _) = tag(": X+")(input)?;
    let (input, x) = u32(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = u32(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, Button { cost, x, y }))
}

fn parse_claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    let (input, button_a) = parse_button(input)?;
    let (input, button_b) = parse_button(input)?;
    let (input, _) = tag("Prize: X=")(input)?;
    let (input, x) = u32(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, y) = u32(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, ClawMachine { buttons: [button_a, button_b], x, y }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<ClawMachine>> {
    terminated(separated_list1(line_ending, parse_claw_machine), eof)(input)
}

fn solve_claw_machine(machine: &ClawMachine) -> Option<usize> {
    let b_start = min(machine.x / machine.buttons[1].x, machine.y / machine.buttons[1].y);
    for b_count in (0..=b_start).rev() {
        let x_remain = machine.x - b_count * machine.buttons[1].x;
        let y_remain = machine.y - b_count * machine.buttons[1].y;
        if x_remain % machine.buttons[0].x == 0 && y_remain % machine.buttons[0].y == 0 
            && x_remain / machine.buttons[0].x == y_remain / machine.buttons[0].y {
                let a_count = x_remain / machine.buttons[0].x;
                return Some((a_count * machine.buttons[0].cost + b_count * machine.buttons[1].cost) as usize);
            }
    }
    None
}
        
fn solve(input: &[ClawMachine]) -> usize {
    input.iter().filter_map(solve_claw_machine).sum::<usize>()
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn run(input: &str) -> usize {
    let (_, machines) = parse_input(input).unwrap();
    solve(&machines)
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
        let (input, machines) = parse_input(&input).unwrap();
        assert_eq!(input, "");
        let machines = black_box(&machines);
        b.iter(|| solve(machines));
    }

    #[test_case("Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
" => 480)]
    fn test(input: &str) -> usize {
        run(input)
    }

    #[test_case("Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
" => vec![
        ClawMachine {
            buttons: [
                Button { cost: 3, x: 94, y: 34 },
                Button { cost: 1, x: 22, y: 67 },
            ],
            x: 8400,
            y: 5400,
        },
        ClawMachine {
            buttons: [
                Button { cost: 3, x: 26, y: 66 },
                Button { cost: 1, x: 67, y: 21 },
            ],
            x: 12748,
            y: 12176,
        },
        ClawMachine {
            buttons: [
                Button { cost: 3, x: 17, y: 86 },
                Button { cost: 1, x: 84, y: 37 },
            ],
            x: 7870,
            y: 6450,
        },
        ClawMachine {
            buttons: [
                Button { cost: 3, x: 69, y: 23 },
                Button { cost: 1, x: 27, y: 71 },
            ],
            x: 18641,
            y: 10279,
        },
    ])]
    fn test_parse_input(input: &str) -> Vec<ClawMachine> {
        parse_input(input).unwrap().1
    }

    #[test_case(ClawMachine {
        buttons: [
            Button { cost: 3, x: 94, y: 34 },
            Button { cost: 1, x: 22, y: 67 },
        ],
        x: 8400,
        y: 5400,
    } => Some(280))]
    #[test_case(ClawMachine {
        buttons: [
            Button { cost: 3, x: 26, y: 66 },
            Button { cost: 1, x: 67, y: 21 },
        ],
        x: 12748,
        y: 12176,
    } => None)]
    #[test_case(ClawMachine {
        buttons: [
            Button { cost: 3, x: 17, y: 86 },
            Button { cost: 1, x: 84, y: 37 },
        ],
        x: 7870,
        y: 6450,
    } => Some(200))]
    #[test_case(ClawMachine {
        buttons: [
            Button { cost: 3, x: 69, y: 23 },
            Button { cost: 1, x: 27, y: 71 },
        ],
        x: 18641,
        y: 10279,
    } => None)]
    fn test_solve_machine(machine: ClawMachine) -> Option<usize> {
        solve_claw_machine(&machine)
    }
}

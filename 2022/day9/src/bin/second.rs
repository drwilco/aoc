use std::{fs, collections::HashSet};

const ROPE_LENGTH: usize = 10;

type Instruction = (char, usize);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn touching(&self, other: &Self) -> bool {
        let x_distance = self.x - other.x;
        let y_distance = self.y - other.y;
        (-1..=1).contains(&x_distance) && (-1..=1).contains(&y_distance)
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            // split at 2, so we get the direction and a space in the first part
            // since we use chars().next() for extracting the direction there which
            // will trim the space
            let (c, n) = line.split_at(2);
            (c.chars().next().unwrap(), n.parse().unwrap())
        })
        .collect()
}

fn do_the_thing(input: &str) -> usize {
    let instructions = parse_instructions(input);
    let mut rope = [Point::default(); ROPE_LENGTH];
    let mut tail_positions = HashSet::<Point>::new();
    for (direction, distance) in instructions {
        for _ in 0..distance {
            match direction {
                'U' => rope[0].y += 1,
                'D' => rope[0].y -= 1,
                'R' => rope[0].x += 1,
                'L' => rope[0].x -= 1,
                _ => panic!("Unknown direction"),
            }
            let mut previous_knot = None;
            for current_knot in &mut rope {
                if let Some(previous_knot) = previous_knot {
                    if !current_knot.touching(&previous_knot) {
                        match current_knot.x.cmp(&previous_knot.x) {
                            std::cmp::Ordering::Less => current_knot.x += 1,
                            std::cmp::Ordering::Greater => current_knot.x -= 1,
                            std::cmp::Ordering::Equal => (),
                        };
                        match current_knot.y.cmp(&previous_knot.y) {
                            std::cmp::Ordering::Less => current_knot.y += 1,
                            std::cmp::Ordering::Greater => current_knot.y -= 1,
                            std::cmp::Ordering::Equal => (),
                        };
                    }
                }
                previous_knot = Some(*current_knot);
            }
            tail_positions.insert(rope[ROPE_LENGTH - 1]);
        }
    }
    tail_positions.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
" => 1)]
#[test_case("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20" => 36)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

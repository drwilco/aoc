use std::{fs, collections::HashSet};

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
    let mut head = Point::default();
    let mut tail = Point::default();
    let mut tail_positions = HashSet::<Point>::new();
    for (direction, distance) in instructions {
        for _ in 0..distance {
            match direction {
                'U' => head.y += 1,
                'D' => head.y -= 1,
                'R' => head.x += 1,
                'L' => head.x -= 1,
                _ => panic!("Unknown direction"),
            }
            if !tail.touching(&head) {
                match tail.x.cmp(&head.x) {
                    std::cmp::Ordering::Less => tail.x += 1,
                    std::cmp::Ordering::Greater => tail.x -= 1,
                    std::cmp::Ordering::Equal => (),
                };
                match tail.y.cmp(&head.y) {
                    std::cmp::Ordering::Less => tail.y += 1,
                    std::cmp::Ordering::Greater => tail.y -= 1,
                    std::cmp::Ordering::Equal => (),
                };
            }
            tail_positions.insert(tail);
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
" => 13)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

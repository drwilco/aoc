use anyhow::Result;
use std::{collections::VecDeque, fs};

fn find_abberation(input: &str, preamble_length: usize) -> u32 {
    let mut ring = VecDeque::<u32>::new();
    input
        .lines()
        .enumerate()
        .find_map(|(index, n)| {
            let n: u32 = n.parse().unwrap();
            if index < preamble_length {
                ring.push_back(n);
                None
            } else {
                for a in &ring {
                    for b in &ring {
                        if a + b == n {
                            ring.push_back(n);
                            ring.pop_front();
                            return None;
                        }
                    }
                }
                Some(n)
            }
        })
        .unwrap()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", find_abberation(&input, 25));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(127, find_abberation(&input, 5));
    }
}

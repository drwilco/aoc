use anyhow::Result;
use std::fs;

fn do_the_thing(input: &str) -> u64 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let schedule = lines[1]
        .split(',')
        .enumerate()
        .filter_map(|(offset, route)| match route {
            "x" => None,
            _ => Some((offset as u64, route.parse::<u64>().unwrap())),
        })
        .collect::<Vec<_>>();

    let (answer, _) = schedule
        .into_iter()
        .fold(None, |acc: Option<(u64, u64)>, (offset, route)| match acc {
            None => Some((offset, route)),
            Some((start, interval)) => {
                let mut t = start;
                while ((t + offset) % route) != 0 {
                    t += interval;
                }
                Some((t, interval * route))
            }
        })
        .unwrap();

    answer
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("7,13,x,x,59,x,31,19" => 1068781)]
    #[test_case("17,x,13,19" => 3417)]
    #[test_case("67,7,59,61" => 754018)]
    #[test_case("67,x,7,59,61" => 779210)]
    #[test_case("67,7,x,59,61" => 1261476)]
    #[test_case("1789,37,47,1889" => 1202161486)]

    fn second(input: &str) -> u64 {
        let input = "\n".to_string() + input;
        do_the_thing(&input)
    }
}

use anyhow::Result;
use std::fs;

fn do_the_thing(input: &str) -> usize {
    let lines = input.split('\n').collect::<Vec<_>>();
    let earliest: usize = lines[0].parse().unwrap();
    let schedule = lines[1]
        .split(',')
        .filter(|&c| c != "x")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    let (fastest_route, fastest_waittime) = schedule
        .into_iter()
        .fold(None, |acc: Option<(usize, usize)>, route| {
            let time_since_last_departure = earliest % route;
            let waittime = route - time_since_last_departure;
            match acc {
                None => Some((route, waittime)),
                Some((_, fastest_waittime)) => {
                    if waittime < fastest_waittime {
                        Some((route, waittime))
                    } else {
                        acc
                    }
                }
            }
        })
        .unwrap();
    fastest_waittime * fastest_route
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

    #[test_case("939
7,13,x,x,59,x,31,19" => 295)]
    fn first(input: &str) -> usize {
        do_the_thing(&input)
    }
}

use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn do_the_thing(input: &str) -> Result<usize> {
    let mut joltages = input
        .lines()
        .map(|n| n.parse::<usize>().unwrap())
        .sorted()
        .collect::<Vec<_>>();
    joltages.insert(0, 0);
    joltages.push(joltages.last().unwrap() + 3);
    let (ones, threes, _) =
        joltages
            .into_iter()
            .fold((0, 0, 0), |(mut ones, mut threes, previous), current| {
                match current - previous {
                    1 => ones += 1,
                    3 => threes += 1,
                    _ => (),
                }
                (ones, threes, current)
            });
    Ok(ones * threes)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("16
10
15
5
1
11
7
19
6
12
4" => 35; "short example")]
    #[test_case("28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3" => 220; "long example")]
    fn first(input: &str) -> usize {
        do_the_thing(&input).unwrap()
    }
}

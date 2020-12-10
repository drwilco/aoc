use anyhow::Result;
use std::fs;

fn do_the_thing(input: &str) -> Result<usize> {
    let mut joltages = input
        .lines()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    joltages.push(0);
    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);
    let mut options = joltages.into_iter().fold(
        Vec::<(usize, usize)>::new(),
        |mut options, current| match current {
            0 => {
                options.push((0, 1));
                options
            }
            _ => {
                let mut options: Vec<_> = options
                    .into_iter()
                    .filter(|(value, _)| *value + 3 >= current)
                    .collect();
                let alternatives = options.iter().map(|(_, alternatives)| *alternatives).sum();
                options.push((current, alternatives));
                options
            }
        },
    );
    Ok(options.pop().unwrap().1)
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
4" => 8; "short example")]
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
3" => 19208; "long example")]
    fn first(input: &str) -> usize {
        do_the_thing(&input).unwrap()
    }
}

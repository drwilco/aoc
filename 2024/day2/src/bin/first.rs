use std::fs;

fn run(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut previous = None;
            for pair in line
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
                .windows(2)
            {
                let a = pair[0];
                let b = pair[1];
                let difference = (a - b).abs();
                if !(1..=3).contains(&difference) {
                    return false;
                }
                let direction = a.cmp(&b);
                if let Some(previous) = previous {
                    if direction != previous {
                        return false;
                    }
                }
                previous = Some(direction);
            }
            true
        })
        .count()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
" => 2)]
    fn test(input: &str) -> usize {
        run(input)
    }
}

use std::fs;

fn report_is_safe(levels: &[i64]) -> bool {
    let mut previous = None;
    for pair in levels.windows(2) {
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
}

fn run(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels = line
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            for i in 0..levels.len() {
                let mut levels = levels.clone();
                levels.remove(i);
                if report_is_safe(&levels) {
                    return true;
                }
            }
            false
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
" => 4; "example")]
    #[test_case("10 9 11 12 13
" => 1; "set direction with what ends up being the element to remove")]
    fn test(input: &str) -> usize {
        run(input)
    }
}

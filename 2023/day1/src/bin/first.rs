use std::fs;

fn do_the_thing(input: &str) -> usize {
    input.lines().map(|line| {
        let digits = line.chars().filter(|c| c.is_digit(10));
        let first: usize = digits.clone().next().unwrap().to_digit(10).unwrap() as usize;
        let last: usize = digits.last().unwrap().to_digit(10).unwrap() as usize;
        first * 10 + last
    }).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet" => 142)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

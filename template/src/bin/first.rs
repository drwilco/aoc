use std::fs;

pub fn run(input: &str) -> i64 {
    0
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Hello world!" => 0)]
    fn test(input: &str) -> i64 {
        run(&input)
    }
}

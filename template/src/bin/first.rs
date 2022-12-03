use std::fs;

fn do_the_thing(input: &str) {
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Hello world!" => ())]
    fn test(input: &str) {
        do_the_thing(&input)
    }
}

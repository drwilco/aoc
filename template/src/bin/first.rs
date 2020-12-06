use anyhow::{Result};
use std::fs;

fn do_the_thing(_input: &str) -> Result<()> {
    Ok(())
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

    #[test_case("Hello world!" => ())]
    fn first(input: &str) {
        do_the_thing(&input).unwrap()
    }
}

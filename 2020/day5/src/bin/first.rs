use anyhow::{anyhow, Result};
use num::{One, Zero};
use std::{
    fs,
    mem::size_of,
    ops::{BitOrAssign, ShlAssign},
};

fn bsp_to_number<T: One + Zero + ShlAssign<u8> + BitOrAssign>(input: &str) -> Result<T> {
    if input.len() / 8 > size_of::<T>() {
        return Err(anyhow!(
            "Input bits ({}) exceed available in {}",
            input.len(),
            std::any::type_name::<T>()
        ));
    }
    let mut result: T = <T>::zero();
    for bit in input.chars() {
        result <<= 1;
        if bit == 'B' || bit == 'R' {
            result |= <T>::one();
        }
    }
    Ok(result)
}

fn highest_seatid(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|seat| bsp_to_number(seat).unwrap())
        .max()
        .unwrap())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", highest_seatid(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("FBFBBFF" => 44)]
    #[test_case("RLR" => 5)]
    fn bsp(input: &str) -> u8 {
        bsp_to_number(input).unwrap()
    }

    #[test_case("FBFBBFFRLR" => 357)]
    #[test_case("BFFFBBFRRR" => 567)]
    #[test_case("FFFBBBFRRR" => 119)]
    #[test_case("BBFFBBFRLL" => 820)]
    fn id(input: &str) -> u32 {
        bsp_to_number(input).unwrap()
    }

    #[test]
    fn highest() {
        assert_eq!(
            highest_seatid(
                "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"
            )
            .unwrap(),
            820
        );
    }
}

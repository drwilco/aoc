use anyhow::{anyhow, Result};
use num::{One, Zero};
use std::{fs, mem::size_of, ops::{BitOrAssign, ShlAssign}};

fn bsp_to_number<T: One + Zero + ShlAssign<u8> + BitOrAssign>(input: &str) -> Result<T> {
    if input.len() / 8 > size_of::<T>() {
        return Err(anyhow!("Input bits ({}) exceed available in {}", input.len(), std::any::type_name::<T>()));
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

fn bsp_seat_to_id(bsp_seat: &str) -> Result<u32> {
    if bsp_seat.len() != 10 {
        Err(anyhow!("BSP seat ID not 10 characters"))
    } else {
        let (row, column) = bsp_seat.split_at(7);
        let column: u32 = bsp_to_number(column)?;
        let row: u32 = bsp_to_number(row)?;
        let id = (row * 8) + column;
        Ok(id)
    }
}

fn highest_seatid(input: &str) -> u32 {
    input.lines().map(|seat| bsp_seat_to_id(seat).unwrap()).max().unwrap()
}

fn find_missing(input: &str) -> Result<u32> {
    let seats = input.lines().map(|seat| bsp_seat_to_id(seat).unwrap()).collect::<Vec<_>>();
    let min = seats.iter().min().unwrap().clone();
    let max = seats.iter().max().unwrap().clone();
    for i in min..=max {
        if !seats.contains(&i) && seats.contains(&(i - 1)) && seats.contains(&(i + 1)) {
            return Ok(i);
        }
    }
    Err(anyhow!("seat not found"))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", find_missing(&input)?);

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
        bsp_seat_to_id(input).unwrap()
    }

    #[test]
    fn highest() {
        assert_eq!(highest_seatid("FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"), 820);
    }
}
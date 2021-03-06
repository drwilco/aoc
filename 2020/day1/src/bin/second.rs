use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?
        .trim()
        .lines()
        .map(|x| x.parse().expect("not a number"))
        .collect::<Vec<u64>>();

    'outer: for a in &input {
        for b in &input {
            for c in &input {
                if a + b + c == 2020 {
                    println!("{:?}", a * b * c);
                    break 'outer;
                }
            }
        }
    }
    Ok(())
}

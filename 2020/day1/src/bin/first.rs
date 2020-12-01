use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?
        .trim()
        .split('\n')
        .map(|x| x.parse().expect("not a number"))
        .collect::<Vec<u64>>();

    for a in &input {
        for b in &input {
            if a + b == 2020 {
                println!("{:?}", a * b);
            }
        }
    }
    Ok(())
}

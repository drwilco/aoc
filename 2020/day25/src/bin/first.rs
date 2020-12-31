use anyhow::Result;

fn find_loop_size(pubkey: usize) -> usize {
    let subject = 7;
    let mut value = 1;
    let mut loop_size = 0;
    while value != pubkey {
        value *= subject;
        value %= 20201227;
        loop_size += 1;
    }
    loop_size
}

fn transform(subject: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject;
        value %= 20201227;
    }
    value
}
fn do_the_thing(pub1: usize, pub2: usize) -> usize {
    let key1 = transform(pub1, find_loop_size(pub2));
    let key2 = transform(pub2, find_loop_size(pub1));
    assert_eq!(key1, key2);
    key1
}

fn main() -> Result<()> {
    println!("{:?}", do_the_thing(13135480, 8821721));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5764801, 17807724 => 14897079)]
    fn first(pub1: usize, pub2: usize) -> usize {
        do_the_thing(pub1, pub2)
    }

    #[test]
    fn find() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);
    }
}

use anyhow::Result;

#[derive(Default)]
struct Node {
    next: Option<usize>,
}

static SIZE: usize = 1000000;

fn do_the_thing(input: &str, moves: usize) -> usize {
    let mut linked_values: Vec<Node> = Vec::with_capacity(SIZE + 1);
    linked_values.resize_with(SIZE + 1, Node::default);

    let numbers = input
        .chars()
        .map(|c| (c.to_digit(10).unwrap()) as usize)
        .collect::<Vec<_>>();
    let max = *numbers.iter().max().unwrap();
    let mut head = 0;
    let tail = numbers
        .into_iter()
        .fold(None, |prev: Option<usize>, current| {
            if let Some(prev) = prev {
                linked_values[prev].next = Some(current);
            } else {
                head = current;
            }
            Some(current)
        })
        .unwrap();

    linked_values[tail].next = Some(max + 1);
    linked_values[max + 1].next = Some(max + 2);
    for num in (max + 2)..SIZE {
        let node = linked_values.get_mut(num).unwrap();
        node.next = Some(num + 1);
    }
    linked_values[SIZE].next = Some(head);

    for _ in 0..moves {
        let mut cursor = head;
        let mut lifted_values = vec![];
        for _ in 0..3 {
            cursor = linked_values[cursor].next.unwrap();
            lifted_values.push(cursor);
        }
        cursor = linked_values[cursor].next.unwrap();
        // now head and cursor are the two numbers before and after
        // the values we just lifted. Connect them to eachother.
        linked_values[head].next = Some(cursor);

        // determine destination value
        let mut destination = head;
        while destination == head || lifted_values.contains(&destination) {
            destination = ((destination + SIZE - 2) % SIZE) + 1;
        }
        // get value after destination to connect to tail of lifted
        let connect = linked_values[destination].next.unwrap();
        let lifted_head = lifted_values[0];
        let lifted_tail = lifted_values[2];
        linked_values[destination].next = Some(lifted_head);
        linked_values[lifted_tail].next = Some(connect);
        head = linked_values[head].next.unwrap();
    }
    let first = linked_values[1].next.unwrap();
    let second = linked_values[first].next.unwrap();
    first * second
}

fn main() -> Result<()> {
    println!("{}", do_the_thing("872495136", 10000000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("389125467", 10000000 => 149245887792)]
    fn first(input: &str, moves: usize) -> usize {
        do_the_thing(&input, moves)
    }
}

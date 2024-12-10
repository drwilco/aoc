#![feature(test)]

use itertools::Itertools;
use std::{cell::Cell, collections::VecDeque, fs, rc::Rc};

#[derive(Clone, Copy, Debug, PartialEq)]
struct File {
    id: usize,
    start: usize,
    length: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Empty {
    start: usize,
    length: usize,
}

#[derive(Debug, PartialEq)]
enum Span {
    File(File),
    Empty(Empty),
}

fn parse_input(input: &str) -> Vec<Span> {
    // Make pairs, so we can enumerate the pair for the file ID
    let start = Rc::new(Cell::new(0));
    input
        .chars()
        .filter_map(|c| c.to_digit(10).and_then(|v| usize::try_from(v).ok()))
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(file_id, lengths)| {
            let start = start.clone();
            lengths.enumerate().map(move |(index, length)| {
                let span_start = start.get();
                start.set(span_start + length);
                if index == 0 {
                    Span::File(File {
                        id: file_id,
                        start: span_start,
                        length,
                    })
                } else {
                    Span::Empty(Empty {
                        start: span_start,
                        length,
                    })
                }
            })
        })
        .collect()
}

fn defrag(spans: Vec<Span>) -> Vec<Span> {
    assert!(!spans.is_empty());
    // This way we don't have to add 1 to the length to get the index. Max size
    // of anything is 9, so use 10 freelists. VecDeque because when we use a
    // span for a smaller file, we create a new free span of a smaller size,
    // which needs to be used first.
    let mut freelists: [VecDeque<Empty>; 10] = Default::default();
    // So we can chain this with freelists
    let mut new_free = VecDeque::new();
    let mut files = Vec::new();
    for span in spans {
        match span {
            Span::File(file) => {
                files.push(file);
            }
            Span::Empty(empty) => {
                freelists[empty.length].push_back(empty);
            }
        }
    }
    for file in files.iter_mut().rev() {
        // Find all freelists that have a span that is as big or bigger than the
        // file
        let empty = freelists[file.length..]
            .iter_mut()
            .filter_map(|list| {
                list.front().copied().and_then(|empty| {
                    // We can have free space to the right of the file, but we
                    // only move left
                    if empty.start < file.start {
                        Some(empty)
                    } else {
                        new_free.append(list);
                        None
                    }
                })
            })
            .reduce(|a, b| if a.start < b.start { a } else { b });
        let Some(mut empty) = empty else {
            if file.length == 1 {
                break;
            }
            continue;
        };
        // Remove the empty span from the freelists
        let actual_empty = freelists[empty.length].pop_front().unwrap();
        debug_assert_eq!(empty, actual_empty);
        debug_assert!(empty.start < file.start);
        file.start = empty.start;
        // Remove space from the empty span
        empty.start += file.length;
        empty.length -= file.length;
        if empty.length > 0 {
            // We will probably have capacity at the front of this but not
            // always, but that means make_contiguous will mostly be free
            freelists[empty.length].push_front(empty);
            freelists[empty.length]
                .make_contiguous()
                .sort_unstable_by_key(|empty| empty.start);
        }
    }
    files.into_iter().map(Span::File).collect()
}

fn checksum(spans: &[Span]) -> usize {
    spans
        .iter()
        .filter_map(|span| match span {
            Span::File(file) => {
                Some((file.start..(file.start + file.length)).map(move |i| i * file.id))
            }
            Span::Empty(_) => None,
        })
        .flatten()
        .sum()
}

fn run(input: &str) -> usize {
    let spans = parse_input(input);
    let spans = defrag(spans);
    checksum(&spans)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", run(&input));
}

#[cfg(test)]
mod tests {
    extern crate test as std_test;
    use super::*;
    use std_test::{black_box, Bencher};
    use test_case::test_case;

    #[bench]
    fn my_benchmark(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = black_box(&input);
        b.iter(|| run(input));
    }

    #[test_case("2333133121414131402" => 2858; "first example")]
    // 54321 => 00000....111..2
    // defrag => 000002111......
    // 012345678
    // 000002111
    // 0*0 + 1*0 + 2*0 + 3*0 + 4*0 + 5*2 + 6*1 + 7*1 + 8*1
    #[test_case("54321" => 31; "second example")]
    fn test(input: &str) -> usize {
        run(input)
    }

    // 00000....111..2
    #[test_case("54321" => vec![
        Span::File(File { id: 0, start: 0, length: 5 }),
        Span::Empty(Empty { start: 5, length: 4 }),
        Span::File(File { id: 1, start: 9, length: 3 }),
        Span::Empty(Empty { start: 12, length: 2 }),
        Span::File(File { id: 2, start: 14, length: 1 }),
    ]; "small test")]
    fn test_parse_input(input: &str) -> Vec<Span> {
        parse_input(input)
    }

    // 000002111......
    #[test_case("54321" => vec![
        Span::File(File { id: 0, start: 0, length: 5 }),
        Span::File(File { id: 1, start: 6, length: 3 }),
        Span::File(File { id: 2, start: 5, length: 1 }),
    ]; "small test")]
    #[test_case("2333133121414131402" => vec![
        Span::File(File { id: 0, start: 0, length: 2 }),
        Span::File(File { id: 1, start: 5, length: 3 }),
        Span::File(File { id: 2, start: 4, length: 1 }),
        Span::File(File { id: 3, start: 15, length: 3 }),
        Span::File(File { id: 4, start: 12, length: 2 }),
        Span::File(File { id: 5, start: 22, length: 4 }),
        Span::File(File { id: 6, start: 27, length: 4 }),
        Span::File(File { id: 7, start: 8, length: 3 }),
        Span::File(File { id: 8, start: 36, length: 4 }),
        Span::File(File { id: 9, start: 2, length: 2 }),
    ]; "example")]
    #[test_case("55" => vec![
        Span::File(File { id: 0, start: 0, length: 5 }),
    ]; "move left only")]
    fn test_defrag(input: &str) -> Vec<Span> {
        let spans = parse_input(input);
        defrag(spans)
    }
}

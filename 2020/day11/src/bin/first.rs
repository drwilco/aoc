use anyhow::Result;
use std::{cmp::min, fs};

#[derive(Debug, PartialEq)]
struct SeatMap(Vec<Vec<char>>);

impl SeatMap {
    fn occupied_around(&self, x: usize, y: usize) -> usize {
        let min_x = if x == 0 { 0 } else { x - 1 };
        let min_y = if y == 0 { 0 } else { y - 1 };
        let max_x = min(x + 1, self.0[0].len() - 1);
        let max_y = min(y + 1, self.0.len() - 1);
        self.0[min_y..=max_y]
            .iter()
            .map(|r| r[min_x..=max_x].iter().filter(|&&s| s == '#').count())
            .sum::<usize>()
            - if self.0[y][x] == '#' { 1 } else { 0 }
    }
}

fn apply_rules(input: &SeatMap) -> SeatMap {
    SeatMap(
        input
            .0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, &seat)| match seat {
                        'L' => {
                            if input.occupied_around(x, y) == 0 {
                                '#'
                            } else {
                                'L'
                            }
                        }
                        '#' => {
                            if input.occupied_around(x, y) >= 4 {
                                'L'
                            } else {
                                '#'
                            }
                        }
                        _ => '.',
                    })
                    .collect()
            })
            .collect(),
    )
}

fn parse_seatmap(input: &str) -> SeatMap {
    SeatMap(
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}

fn do_the_thing(input: &str) -> usize {
    let mut map = parse_seatmap(input);
    while {
        let oldmap = map;
        map = apply_rules(&oldmap);
        map != oldmap
    } {}
    map.0
        .into_iter()
        .map(|r| r.into_iter().filter(|&s| s == '#').count())
        .sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INITIAL: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    static EXAMPLE_FIRST_ITERATION: &str = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    static EXAMPLE_SECOND_ITERATION: &str = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";

    #[test]
    fn test_rules() {
        let map1 = parse_seatmap(EXAMPLE_INITIAL);
        let map2 = parse_seatmap(EXAMPLE_FIRST_ITERATION);
        let map3 = parse_seatmap(EXAMPLE_SECOND_ITERATION);
        assert_eq!(apply_rules(&map1), map2);
        assert_eq!(apply_rules(&map2), map3);
    }

    #[test]
    fn test_occupied() {
        let map1 = parse_seatmap(EXAMPLE_INITIAL);
        let map2 = parse_seatmap(EXAMPLE_FIRST_ITERATION);
        assert_eq!(map1.occupied_around(0, 0), 0);
        assert_eq!(map2.occupied_around(0, 0), 2);
        assert_eq!(map2.occupied_around(9, 9), 2);
        assert_eq!(map2.occupied_around(4, 8), 8);
    }

    #[test]
    fn first() {
        assert_eq!(do_the_thing(EXAMPLE_INITIAL), 37);
    }
}

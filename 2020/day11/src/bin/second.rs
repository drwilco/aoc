use anyhow::Result;
use itertools::Itertools;
use std::fs;

#[derive(Debug, PartialEq)]
struct SeatMap(Vec<Vec<char>>);

impl SeatMap {
    fn occupied_in_direction(&self, x: usize, y: usize, x_offset: isize, y_offset: isize) -> bool {
        let mut x = x as isize;
        let mut y = y as isize;
        let width = self.0[0].len() as isize;
        let height = self.0.len() as isize;
        loop {
            x += x_offset;
            y += y_offset;
            if x < 0 || x >= width || y < 0 || y >= height {
                return false;
            }
            match self.0[y as usize][x as usize] {
                'L' => return false,
                '#' => return true,
                _ => (),
            }
        }
    }

    fn occupied_around(&self, x: usize, y: usize) -> usize {
        let directions: Vec<(isize, isize)> = vec![-1, 0, 1]
            .into_iter()
            .cartesian_product(vec![-1, 0, 1])
            .filter(|d| *d != (0, 0))
            .collect();
        directions
            .into_iter()
            .filter(|(x_offset, y_offset)| self.occupied_in_direction(x, y, *x_offset, *y_offset))
            .count()
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
                            if input.occupied_around(x, y) >= 5 {
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

    static EXAMPLE_SECOND_ITERATION: &str = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#";

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
        assert_eq!(map2.occupied_around(0, 0), 3);
        assert_eq!(map2.occupied_around(9, 9), 3);
        assert_eq!(map2.occupied_around(4, 6), 7);
    }

    #[test]
    fn first() {
        assert_eq!(do_the_thing(EXAMPLE_INITIAL), 26);
    }
}

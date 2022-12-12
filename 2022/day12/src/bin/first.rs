use std::{fs, collections::VecDeque};

#[derive(Clone, Debug, Default)]
struct Square {
    height: usize,
    start: bool,
    end: bool,
    steps_to_end: Option<usize>,
}

#[derive(Debug, Default)]
struct Grid {
    squares: Vec<Vec<Square>>,
    width: usize,
    length: usize,
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
}

impl Grid {
    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.length - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }
}

// parse character to Square, a-z to 0-25
// S is start and height 0, E is end and height 25
fn parse_square(c: char) -> Square {
    match c {
        'S' => Square {
            height: 0,
            start: true,
            end: false,
            ..Default::default()
        },
        'E' => Square {
            height: 25,
            start: false,
            end: true,
            ..Default::default()
        },
        _ => Square {
            height: c as usize - ('a' as usize),
            ..Default::default()
        },
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut grid = Grid::default();
    // we want to index with [x][y] so we need to transpose
    for _ in 0..input.lines().next().unwrap().chars().count() {
        grid.squares.push(Vec::new());
    }
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut square = parse_square(c);
            if square.start {
                grid.start = Some((x, y));
            }
            if square.end {
                grid.end = Some((x, y));
                square.steps_to_end = Some(0);
            }
            grid.squares[x].push(square);
        }
    }
    grid.width = grid.squares.len();
    grid.length = grid.squares[0].len();
    grid
}

fn do_the_thing(input: &str) -> usize {
    let mut grid = parse_grid(input);
    // we should have an end at this point, so unwrap blindly
    let (x, y) = grid.end.unwrap();
    let mut queue = VecDeque::from([(x, y)]);
    while let Some((square_x, square_y)) = queue.pop_front() {
        // clone so we don't have a borrow on grid.squares
        let square = grid.squares[square_x][square_y].clone();
        assert!(square.steps_to_end.is_some());
        for (neighbor_x, neighbor_y) in grid.get_neighbors(square_x, square_y) {
            let neighbor = &mut grid.squares[neighbor_x][neighbor_y];
            if neighbor.steps_to_end.is_some() {
                // this square has already been processed so either has a
                // shorter path already or same length. Either way, it has
                // already been added to the queue at some point
                assert!(neighbor.steps_to_end.unwrap() <= square.steps_to_end.unwrap() + 1);
                continue;
            } else {
                // this square has not been processed yet, since it has no steps_to_end
                // said neighbor can only reach current square if it has
                // a height that's at most 1 lower than current square
                // or is higher than current square
                if neighbor.height + 1 < square.height {
                    continue;
                }
                neighbor.steps_to_end = Some(square.steps_to_end.unwrap() + 1);
            }
            if neighbor.start {
                return neighbor.steps_to_end.unwrap();
            }
            queue.push_back((neighbor_x, neighbor_y));
        }
    }
    let start = grid.start.unwrap();
    grid.squares[start.0][start.1].steps_to_end.unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
" => 31)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

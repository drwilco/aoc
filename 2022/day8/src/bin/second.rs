use std::fs;

fn check_sightline(grid: &Vec<Vec<u8>>, x: usize, y: usize, x_delta: isize, y_delta: isize) -> usize {
    let width = grid.len();
    let length = grid[0].len();
    let own_height = grid[x][y];
    let mut x = x as isize;
    let mut y = y as isize;
    let mut score = 0;
    loop {
        x += x_delta;
        y += y_delta;
        if x < 0 || x >= width as isize || y < 0 || y >= length as isize {
            break;
        }
        score += 1;
        let x = x as usize;
        let y = y as usize;
        let height = grid[x][y];
        if height >= own_height {
            break;
        }
    }
    score
}

fn do_the_thing(input: &str) -> usize {
    let tree_grid = input.lines().map(|line| {
        line.chars().map(|c| c as u8 - 48).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let width = tree_grid.len();
    let length = tree_grid[0].len();
    // since the edges will always have a score of 0, we can skip them
    (1..(width - 1)).into_iter().map(|x| {
        (1..(length - 1)).into_iter().map(|y| {
            [
                check_sightline(&tree_grid, x, y, -1, 0),
                check_sightline(&tree_grid, x, y, 1, 0),
                check_sightline(&tree_grid, x, y, 0, -1),
                check_sightline(&tree_grid, x, y, 0, 1),
            ].into_iter().product()
        }).max().unwrap()
    }).max().unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", do_the_thing(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("30373
25512
65332
33549
35390
" => 8)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

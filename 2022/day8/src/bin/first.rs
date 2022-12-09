use std::fs;

struct Remap {
    axis_swap: bool,
    direction_swap: bool,
}

fn view(grid: &Vec<Vec<u8>>, remap: Remap) -> Vec<Vec<bool>> {
    let width = grid.len();
    let length = grid[0].len();
    assert_eq!(width, length);
    let mut view = Vec::new();
    view.resize_with(width, || {
        let mut row = Vec::new();
        row.resize_with(length, || false);
        row
    });
    for x in 0..width {
        let mut highest = None;
        (0..length).into_iter().find(|&y| {
            let (x, y) = if remap.axis_swap {
                (y, x)
            } else {
                (x, y)
            };
            let (x, y) = if remap.direction_swap {
                (width - x - 1, length - y - 1)
            } else {
                (x, y)
            };
            let height = grid[x][y];
            let visible = match highest {
                None => true,
                Some(heighest_so_far) => height > heighest_so_far,
            };
            if visible {
                highest = Some(height);
            }
            view[x][y] = visible;
            // stop scanning (return true), if we've reached the highest
            // possible height, which is 9.
            height == 9
        });
    }    
    view
}

fn do_the_thing(input: &str) -> usize {
    let tree_grid = input.lines().map(|line| {
        line.chars().map(|c| c as u8 - 48).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    
    // get a grid of bools for eache of the 4 views
    let views = [
        view(&tree_grid, Remap { axis_swap: false, direction_swap: false }),
        view(&tree_grid, Remap { axis_swap: true, direction_swap: false }),
        view(&tree_grid, Remap { axis_swap: false, direction_swap: true }),
        view(&tree_grid, Remap { axis_swap: true, direction_swap: true }),
    ];
    // zip all the views together, and count the number of true values
    views[0].iter().zip(views[1].iter()).zip(views[2].iter()).zip(views[3].iter()).map(|(((a, b), c), d)| {
        a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter()).map(|(((a, b), c), d)| {
            (*a || *b || *c || *d) as usize
        }).sum::<usize>()
    }).sum::<usize>()
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
" => 21)]
    fn test(input: &str) -> usize {
        do_the_thing(&input)
    }
}

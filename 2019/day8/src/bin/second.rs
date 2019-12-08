use std::fs;
use std::io;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() -> io::Result<()> {
  let input = fs::read_to_string("input.txt")?;
  let mut input = input.trim().chars().peekable();
  let mut layers = Vec::new();

  while input.peek().is_some() {
    let mut layer: [[char; HEIGHT]; WIDTH] = [['x'; HEIGHT]; WIDTH];
    for y in 0..HEIGHT {
      for x in 0..WIDTH {
        layer[x][y] = input.next().unwrap();
      }
    }
    layers.push(layer);
  }

  let mut output: [[char; HEIGHT]; WIDTH] = [['x'; HEIGHT]; WIDTH];
  for y in 0..HEIGHT {
    for x in 0..WIDTH {
      for l in 0..layers.len() {
        match layers[l][x][y] {
          '0' => {
            output[x][y] = ' ';
            break;
          },
          '1' => {
            output[x][y] = 'X';
            break;
          },
          '2' => (),
          _ => panic!("unknown color"),
        } 
      }
    }
  }
  for y in 0..HEIGHT {
    for x in 0..WIDTH {
      print!("{}", output[x][y]);
    }
    println!();
  }
  Ok(())
}

use std::{fs, collections::HashMap};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1, line_ending},
    combinator::map,
    IResult, multi::separated_list1,
};

struct Glimpse {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    glimpses: Vec<Glimpse>,
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)
}

fn parse_component(input: &str) -> IResult<&str, (&str, usize)> {
    let (input, amount) = parse_number(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;
    Ok((input, (color, amount)))
}

fn parse_glimpse(input: &str) -> IResult<&str, Glimpse> {
    let (input, colors) = separated_list1(tag(", "), parse_component)(input)?;
    let colors = colors.into_iter().collect::<HashMap<_, _>>();
    let red = *colors.get("red").unwrap_or(&0);
    let green = *colors.get("green").unwrap_or(&0);
    let blue = *colors.get("blue").unwrap_or(&0);
    Ok((input, Glimpse { red, green, blue }))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, _) = parse_number(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, glimpses) = separated_list1(tag("; "), parse_glimpse)(input)?;
    Ok((input, Game { glimpses }))
}

fn do_the_thing(input: &str) -> IResult<&str, usize> {
    let (input, games) = separated_list1(line_ending, parse_game)(input)?;
    let (input, _) = line_ending(input)?;
    assert!(input.is_empty());
    Ok((input, games.into_iter().map(|game| {
        let max_red = game.glimpses.iter().map(|g| g.red).max().unwrap();
        let max_green = game.glimpses.iter().map(|g| g.green).max().unwrap();
        let max_blue = game.glimpses.iter().map(|g| g.blue).max().unwrap();
        max_red * max_green * max_blue
    }).sum()))
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (_, result) = do_the_thing(&input).unwrap();
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
" => 2286)]
    fn test(input: &str) -> usize {
        do_the_thing(&input).unwrap().1
    }
}

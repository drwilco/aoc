use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending},
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};
use std::{collections::HashSet, fs, iter::FromIterator};

#[derive(Debug)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

impl<'a> Food<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (input, ingredients) = separated_list1(char(' '), alpha1)(input)?;
        let (input, allergens) = opt(preceded(
            tag(" (contains "),
            terminated(separated_list1(tag(", "), alpha1), char(')')),
        ))(input)?;
        let allergens = match allergens {
            Some(allergens) => HashSet::from_iter(allergens),
            None => HashSet::new(),
        };
        Ok((
            input,
            Self {
                ingredients: HashSet::from_iter(ingredients),
                allergens,
            },
        ))
    }
}

fn do_the_thing(input: &str) -> IResult<&str, usize> {
    let (_, foods): (_, Vec<Food>) = separated_list1(line_ending, Food::parse)(input)?;
    let (mut ingredients, mut allergens): (HashSet<&str>, HashSet<&str>) = foods.iter().fold(
        (HashSet::new(), HashSet::new()),
        |(mut all_ingredients, mut all_allergens), food| {
            all_ingredients.extend(&food.ingredients);
            all_allergens.extend(&food.allergens);
            (all_ingredients, all_allergens)
        },
    );
    while !allergens.is_empty() {
        for allergen in &allergens.clone() {
            let linked_ingredients = foods
                .iter()
                .filter(|food| food.allergens.contains(allergen))
                .fold(ingredients.clone(), |acc, food| {
                    acc.intersection(&food.ingredients).copied().collect()
                });
            if linked_ingredients.len() == 1 {
                allergens.remove(allergen);
                ingredients = ingredients
                    .difference(&linked_ingredients)
                    .copied()
                    .collect();
            }
        }
    }
    Ok((
        input,
        ingredients
            .into_iter()
            .map(|ingredient| {
                foods
                    .iter()
                    .filter(|food| food.ingredients.contains(ingredient))
                    .count()
            })
            .sum(),
    ))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", do_the_thing(&input).unwrap().1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)" => 5; "example")]
    fn first(input: &str) -> usize {
        do_the_thing(&input).unwrap().1
    }
}

use std::io;
use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1, alpha1};
use nom::combinator::map;
use nom::sequence::pair;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Debug, Default, Clone)]
struct Input {
  name: String,
  count: usize,
}

#[derive(Debug, Default)]
struct Reaction {
  output_count: usize,
  inputs: Vec<Input>,
  spares: usize,
}

const MAX_ORE: usize = 1000000000000;

fn parse_num(input: &str) -> IResult<&str, usize> {
  map(digit1, |digit_str: &str| digit_str.parse::<usize>().unwrap())(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
  let (input, count) = parse_num(input)?;
  let (input, _) = space1(input)?;
  let (input, name) = alpha1(input)?;
  Ok((input, Input{name: name.to_string(), count}))
}

fn parse_reaction(input: &str) -> IResult<&str, (String, Reaction)> {
  let (input, inputs) = separated_list1(pair(tag(","), space1), parse_input)(input)?;
  let (input, _) = space1(input)?;
  let (input, _) = tag("=>")(input)?;
  let (input, _) = space1(input)?;
  let (input, output_count) = parse_num(input)?;
  let (input, _) = space1(input)?;
  let (input, name) = alpha1(input)?;
  Ok((input, (name.to_string(), Reaction{output_count, inputs, spares: 0})))
}

fn string_to_reactions(string: &str) -> HashMap<String, Reaction> {
  let mut reactions: HashMap<String, Reaction> = HashMap::new();
  for line in string.lines() {
    let (_, (name, reaction)) = parse_reaction(line).unwrap();
    reactions.insert(name, reaction);
  }
  reactions
}

fn breakdown(reactions: &mut HashMap<String, Reaction>, output: &String, mut needed: usize) -> usize {
  if output == "ORE" {
    return needed;
  }
  let reaction = reactions.get(output).unwrap();
  if needed <= reaction.spares {
    if let Some(reaction) = reactions.get_mut(output) {
      reaction.spares -= needed;
      return 0;
    } else {
      panic!("No reaction for {} found", output);
    }
  }
  if reaction.spares > 0 {
    needed -= reaction.spares;
  }
  let mut multiplier: usize;
  if reaction.output_count >= needed {
    multiplier = 1;
  } else {
    multiplier = needed / reaction.output_count;
    if needed % reaction.output_count > 0 {
      multiplier += 1;
    }
  }
  let inputs = reaction.inputs.clone(); 
  let total = inputs.iter().map(|input| breakdown(reactions, &input.name, input.count * multiplier)).sum();
  if let Some(reaction) = reactions.get_mut(output) {
    reaction.spares = (reaction.output_count * multiplier) - needed;
  } else {
    panic!("No reaction for {} found", output);
  }
  total
}

fn approx_breakdown(reactions: &HashMap<String, Reaction>, output: &String, needed: f32) -> f32 {
  if output == "ORE" {
    return needed;
  }
  let reaction = reactions.get(output).unwrap();
  let multiplier = needed / reaction.output_count as f32;
  let inputs = reaction.inputs.clone(); 
  inputs.iter().map(|input| approx_breakdown(reactions, &input.name, input.count as f32 * multiplier)).sum()
}

fn output_for_size(mut reactions: &mut HashMap<String, Reaction>, output: &String) -> usize {
  let mut needed = MAX_ORE / approx_breakdown(&reactions, output, 1 as f32) as usize;
  println!("starting at {}", needed);
  let ore = breakdown(&mut reactions, output, needed);
  let mut adjustment: usize = 0;
  println!("ore count: {:?}", ore);
  match ore.cmp(&MAX_ORE) {
    Ordering::Less => {
      adjustment = 1;
      while breakdown(&mut reactions, output, needed) < MAX_ORE {
        needed += 1;
      }
    },
    Ordering::Greater => {
      while breakdown(&mut reactions, output, needed) > MAX_ORE {
        needed -= 1;
      }
    }
    Ordering::Equal => return needed,
  }
  needed - adjustment
}

fn main() -> io::Result<()> {
  let mut reactions = string_to_reactions(&fs::read_to_string("input.txt").unwrap());
  println!("{}", output_for_size(&mut reactions, &"FUEL".to_string()));
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case("10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL", 1, "FUEL" => 31 ; "example 1")]
  #[test_case("9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL", 1, "FUEL" => 165 ; "example 2")]
  #[test_case("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT", 1, "FUEL" => 13312 ; "example 3")]
  #[test_case("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF", 1, "FUEL" => 180697 ; "example 4")]
  #[test_case("171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX", 1, "FUEL" => 2210736 ; "example 5")]
  fn test(input: &str, needed: usize, name: &str) -> usize {
    let mut reactions = string_to_reactions(input);
    breakdown(&mut reactions, &name.to_string(), needed)
  }

  #[test_case("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT", "FUEL" => 82892753 ; "example 1")]
  #[test_case("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF", "FUEL" => 5586022 ; "example 2")]
  #[test_case("171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX", "FUEL" => 460664 ; "example 3")]
  fn test_part2(input: &str, name: &str) -> usize {
    let mut reactions = string_to_reactions(input);
    output_for_size(&mut reactions, &name.to_string())
  }
}
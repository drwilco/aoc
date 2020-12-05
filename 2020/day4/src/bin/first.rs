use std::{fs, io};

fn check_passport(passport: &str) -> bool {
    let passport = passport
        .split(|c| c == ' ' || c == '\n')
        .map(|s| s.split(':').next().unwrap())
        .collect::<Vec<_>>();
    for required in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        if !passport.contains(&required) {
            return false;
        }
    }
    true
}

fn check_passports(passports: &str) -> usize {
    passports
        .split("\n\n")
        .filter(|p| check_passport(p))
        .count()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", check_passports(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in" => 2)]
    #[test_case("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm" => 1)]
    #[test_case("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929" => 0)]
    #[test_case("hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm" => 1)]
    #[test_case("hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in" => 0)]
    fn first(input: &str) -> usize {
        check_passports(input)
    }
}

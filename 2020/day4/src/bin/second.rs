use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, io};

static EYECOLORS: &[&str; 7] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

lazy_static! {
    static ref HEIGHT: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    static ref HAIRCOLORS: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PASSPORTID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

fn check_passport(passport: &str) -> bool {
    let passport = passport
        .split(|c| c == ' ' || c == '\n')
        .filter_map(|s| {
            if s.contains(':') {
                let mut kv = s.split(':');
                Some((kv.next().unwrap(), kv.next().unwrap()))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    match passport.get("byr").map(|byr| u32::from_str(byr)) {
        Some(Ok(byr)) if byr >= 1920 && byr <= 2002 => {},
        _ => return false,
    }
    match passport.get("iyr").map(|iyr| u32::from_str(iyr)) {
        Some(Ok(iyr)) if iyr >= 2010 && iyr <= 2020 => (),
        _ => return false,
    }
    match passport.get("eyr").map(|eyr| u32::from_str(eyr)) {
        Some(Ok(eyr)) if eyr >= 2020 && eyr <= 2030 => (),
        _ => return false,
    }
    match passport.get("hgt").map(|hgt| HEIGHT.captures(hgt)) {
        Some(Some(captures)) => match (&captures[2], u32::from_str(&captures[1])) {
            ("in", Ok(inches)) if inches >= 59 && inches <= 76 => (),
            ("cm", Ok(cm)) if cm >= 150 && cm <= 193 => (),
            _ => return false,
        },
        _ => return false,
    }
    match passport.get("hcl") {
        Some(hcl) if HAIRCOLORS.is_match(hcl) => (),
        _ => return false,
    }
    match passport.get("ecl") {
        Some(ecl) if EYECOLORS.contains(ecl) => (),
        _ => return false,
    }
    match passport.get("pid") {
        Some(pid) if PASSPORTID.is_match(pid) => (),
        _ => return false,
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

    #[test_case("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007" => 0; "invalid passports")]
    #[test_case("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719" => 4; "valid passports")]
    fn second(input: &str) -> usize {
        check_passports(input)
    }
}

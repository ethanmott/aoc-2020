use common::files;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const VALID_EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

lazy_static! {
    static ref PASSPORT_SEPARATOR: Regex = Regex::new(r"\n\n|\r\n\r\n").unwrap();
}

fn main() {
    let input = files::get_file_as_string("input/day04.txt");

    println!("part1: {:?}", solve_part1(&input));
    println!("part2: {:?}", solve_part2(&input));
}

fn solve_part1(input: &String) -> usize {
    PASSPORT_SEPARATOR.split(input)
        .map(passport_from_str)
        .filter(|p| has_fields(p, REQUIRED_FIELDS))
        .count()
}

fn solve_part2(input: &String) -> usize {
    PASSPORT_SEPARATOR.split(input)
        .map(passport_from_str)
        .filter(|p| has_fields(p, REQUIRED_FIELDS))
        .filter(|p| valid_birth_year(p.get("byr").unwrap()))
        .filter(|p| valid_issue_year(p.get("iyr").unwrap()))
        .filter(|p| valid_expiration_year(p.get("eyr").unwrap()))
        .filter(|p| valid_height(p.get("hgt").unwrap()))
        .filter(|p| valid_hair_color(p.get("hcl").unwrap()))
        .filter(|p| valid_eye_color(p.get("ecl").unwrap()))
        .filter(|p| valid_passport_id(p.get("pid").unwrap()))
        .count()
}

type Passport<'a> = HashMap<&'a str, &'a str>;

// tried implementing FromStr but was stuck in lifetime hell
fn passport_from_str(s: &str) -> Passport {
    s.split_whitespace()
        .map(|kv| {
            let split: Vec<&str> = kv.split(":").collect();

            (split[0], split[1])
        })
        .collect()
}

fn has_fields(passport: &Passport, fields: &[&str]) -> bool {
    fields.into_iter()
        .all(|&field| passport.contains_key(field))
}

fn valid_birth_year(value: &str) -> bool {
    let birth_year = value.parse::<u32>().unwrap_or(0);

    value.len() == 4 && birth_year >= 1920 && birth_year <= 2002
}

fn valid_issue_year(value: &str) -> bool {
    let issue_year = value.parse::<u32>().unwrap_or(0);

    value.len() == 4 && issue_year >= 2010 && issue_year <= 2020
}

fn valid_expiration_year(value: &str) -> bool {
    let expiration_year = value.parse::<u32>().unwrap_or(0);

    value.len() == 4 && expiration_year >= 2020 && expiration_year <= 2030
}

fn valid_height(value: &str) -> bool {
    if value.ends_with("cm") {
        let height_cm = value.trim_end_matches("cm").parse::<u32>().unwrap_or(0);

        height_cm >= 150 && height_cm <= 193
    } else if value.ends_with("in") {
        let height_in = value.trim_end_matches("in").parse::<u32>().unwrap_or(0);

        height_in >= 59 && height_in <= 76
    } else {
        false
    }
}

fn valid_hair_color(value: &str) -> bool {
    Regex::new(r"^#[a-z0-9]{6}$").unwrap().is_match(value)
}

fn valid_eye_color(value: &str) -> bool {
    VALID_EYE_COLORS.contains(&value)
}

fn valid_passport_id(value: &str) -> bool {
    Regex::new(r"^[0-9]{9}$").unwrap().is_match(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = r#"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        "#.to_string();

        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn test_solve_part2() {
        let input = r#"
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946

            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007

            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f

            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022

            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "#.to_string();

        assert_eq!(solve_part2(&input), 4);
    }
}

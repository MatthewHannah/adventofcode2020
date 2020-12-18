use std::collections::HashMap;

type Passport<'a> = HashMap<&'a str, &'a str>;
type ValidatorMap<'a> = HashMap<&'a str, Validator>;
type Validator = fn(&str) -> bool;

fn parse_passport(pass_str: &str) -> Passport {
    pass_str.split_whitespace()
            .map(|pair| {
                let mut it = pair.split(":");
                (it.next().unwrap(), it.next().unwrap())
            })
            .collect()
}

fn no_field_validation(_value: &str) -> bool {
    true
}

fn validate_year(value: &str, min: i32, max: i32) -> bool {
    value.parse::<i32>().map_or(false, |val| val >= min && val <= max)
}

fn validate_birth_year(value: &str) -> bool {
    validate_year(value, 1920, 2002)
}

fn validate_issue_year(value: &str) -> bool {
    validate_year(value, 2010, 2020)
}

fn validate_expiration_year(value: &str) -> bool {
    validate_year(value, 2020, 2030)
}

fn validate_height(value: &str) -> bool {
    if let Some(inch_end) = value.find("in") {
        let (inch_val, _) = value.split_at(inch_end);
        let inches = inch_val.parse::<i32>();
        return inches.map_or(false, |x| x >= 59 && x <= 76);
    } else if let Some(cm_end) = value.find("cm") {
        let (cm_val, _) = value.split_at(cm_end);
        let cms = cm_val.parse::<i32>();
        return cms.map_or(false, |x| x >= 150 && x <= 193);
    } else {
        return false;
    }
}

fn validate_hair_color(value: &str) -> bool {
    let mut chars = value.chars();
    let starts_with_pound = chars.next().map_or(false, |c| c == '#');
    let valid_digits = chars.take(7) // take 7, expect count to be 6 to ensure only 6 digits
                            .map(|c| c.is_ascii_hexdigit())
                            .filter(|&b| b)
                            .count() == 6;
    starts_with_pound && valid_digits
}

fn validate_eye_color(value: &str) -> bool {
    match value {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false
    }
}

fn validate_passport_id(value: &str) -> bool {
    value.chars()
         .map(|c| (1, c.is_ascii_digit()))
         .fold((0, true), |(sum, digits), (i, is_digit)| (sum + i, digits && is_digit))
         == (9, true)
}

fn validate_passport(passport: Passport, valid_map: &ValidatorMap) -> bool {
    valid_map.iter()
             .map(|(key, validator)| passport.get(key).map_or(false, |&s| validator(s)))
             .fold(true, |acc, x| acc && x)
}

fn count_passports(passports: &str, valid_map: &ValidatorMap) -> usize {
    passports.split("\n\n")
             .map(|passport| validate_passport(parse_passport(passport), valid_map))
             .filter(|&x| x)
             .count()
}

fn main() {
    let test_input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    let mut empty_validators : ValidatorMap = HashMap::new(); 
    empty_validators.insert("byr", no_field_validation);
    empty_validators.insert("iyr", no_field_validation);
    empty_validators.insert("eyr", no_field_validation);
    empty_validators.insert("hgt", no_field_validation);
    empty_validators.insert("hcl", no_field_validation);
    empty_validators.insert("ecl", no_field_validation);
    empty_validators.insert("pid", no_field_validation);

    println!("Test valid count: {:?}", count_passports(&test_input, &empty_validators));

    let real_input = include_str!("day4.txt");
    println!("Real valid count: {}", count_passports(&real_input, &empty_validators));

    let mut real_validators : ValidatorMap = HashMap::new();
    real_validators.insert("byr", validate_birth_year);
    real_validators.insert("iyr", validate_issue_year);
    real_validators.insert("eyr", validate_expiration_year);
    real_validators.insert("hgt", validate_height);
    real_validators.insert("hcl", validate_hair_color);
    real_validators.insert("ecl", validate_eye_color);
    real_validators.insert("pid", validate_passport_id);

    println!("Part 2 validation count : {}", count_passports(&real_input, &real_validators));
}
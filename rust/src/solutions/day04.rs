use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    println!("Day 4");

    let raw_input = adventlib::read_input_raw("day04input.txt");
    let passport_strings: Vec<_> = raw_input.split("\n\n").collect();

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid_count = 0;
    for passport in passport_strings.iter() {
        let required_set: HashSet<_> = required_fields.iter().collect();
        let included_field_keys = parse_field_keys(passport);
        let included_set: HashSet<_> = included_field_keys.iter().collect();

        if required_set.difference(&included_set).count() == 0 {
            valid_count += 1;
        }
    }

    println!("Output (part 1): {}", valid_count);

    let mut valid_count = 0;
    for passport in passport_strings.iter() {
        let included_fields: HashMap<_, _> = parse_fields(passport).iter().map(|&x| x).collect();

        let is_valid = required_fields.iter().all(|&field_key| {
            included_fields
                .get(field_key)
                .and_then(|&val| check_field_rules(field_key, val))
                .unwrap_or(false)
        });

        if is_valid {
            valid_count += 1;
        }
    }
    println!("Output (part 2): {}", valid_count);
}

fn parse_field_keys<'a>(line: &'a str) -> Vec<&'a str> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"\b(\w+):.*?\b").unwrap();
    }
    PATTERN
        .captures_iter(line)
        .filter_map(|cap| match cap.get(1) {
            Some(key) => Some(key.as_str()),
            _ => None,
        })
        .collect()
}

fn parse_fields<'a>(line: &'a str) -> Vec<(&'a str, &'a str)> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"\b(\w+):([#\w]*)\b").unwrap();
    }
    PATTERN
        .captures_iter(line)
        .filter_map(|cap| match (cap.get(1), cap.get(2)) {
            (Some(key), Some(value)) => Some((key.as_str(), value.as_str())),
            _ => None,
        })
        .collect()
}

pub fn check_field_rules(field_key: &str, field_value: &str) -> Option<bool> {
    // println!("checking {} {}", field_key, field_value);
    match field_key {
        "byr" => is_year_between(field_value, 1920, 2002),
        "iyr" => is_year_between(field_value, 2010, 2020),
        "eyr" => is_year_between(field_value, 2020, 2030),
        "hgt" => is_valid_height(field_value),
        "hcl" => is_valid_hair_color(field_value),
        "ecl" => is_valid_eye_color(field_value),
        "pid" => is_valid_id(field_value),
        _ => Some(false),
    }
}

fn is_year_between(value: &str, low: u16, high: u16) -> Option<bool> {
    is_num_between(value, low, high)
}

fn is_num_between(value: &str, low: u16, high: u16) -> Option<bool> {
    value.parse::<u16>().ok().map(|yr| low <= yr && yr <= high)
}

fn is_valid_height(value: &str) -> Option<bool> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
    }

    PATTERN
        .captures(value)
        .map(|cap| {
            let unit = cap.get(2).map(|m| m.as_str());
            match (cap.get(1), unit) {
                (Some(num), Some("in")) => is_num_between(num.as_str(), 59, 76),
                (Some(num), Some("cm")) => is_num_between(num.as_str(), 150, 193),
                _ => Some(false),
            }
        })
        .flatten()
}

fn is_valid_eye_color(value: &str) -> Option<bool> {
    let is_in_list = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value);
    Some(is_in_list)
}

fn is_valid_hair_color(value: &str) -> Option<bool> {
    Some(matches_pattern(value, r"^#[0-9a-f]{6}$"))
}

fn is_valid_id(value: &str) -> Option<bool> {
    Some(matches_pattern(value, r"^\d{9}$"))
}

fn matches_pattern(value: &str, pattern: &str) -> bool {
    let re = Regex::new(pattern).unwrap();
    re.is_match(value)
}

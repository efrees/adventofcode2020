use lazy_static::lazy_static;

use regex::Regex;

struct PasswordSpec {
    min: i32,
    max: i32,
    character: char,
}

pub fn solve() {
    println!("Day 2");

    let input_lines = adventlib::read_input_lines("day02input.txt");

    let parsed_lines: Vec<_> = input_lines
        .iter()
        .map(|line| parse_password(line))
        .collect();

    let output = parsed_lines
        .iter()
        .filter(|(spec, pw)| matches_spec_one(&spec, &pw))
        .count();

    println!("Output (part 1): {}", output);

    let output = parsed_lines
        .iter()
        .filter(|(spec, pw)| matches_spec_two(&spec, &pw))
        .count();

    println!("Output (part 2): {}", output);
}

fn parse_password(line: &str) -> (PasswordSpec, String) {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    }
    let captures = PATTERN.captures(line).unwrap();
    let spec = PasswordSpec {
        min: captures[1].parse().unwrap(),
        max: captures[2].parse().unwrap(),
        character: captures[3].chars().nth(0).unwrap(),
    };
    return (spec, captures[4].to_string());
}

fn matches_spec_one(spec: &PasswordSpec, password: &str) -> bool {
    let char_count = password.chars().filter(|&c| c == spec.character).count() as i32;
    return spec.min <= char_count && spec.max >= char_count;
}

fn matches_spec_two(spec: &PasswordSpec, password: &str) -> bool {
    let first_position = password.chars().nth(spec.min as usize - 1).unwrap();
    let second_position = password.chars().nth(spec.max as usize - 1).unwrap();
    return (first_position == spec.character) ^ (second_position == spec.character);
}

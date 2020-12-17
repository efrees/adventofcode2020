use lazy_static::lazy_static;
use regex::Regex;

struct FieldRule {
    field_name: String,
    ranges: Vec<ValueRange>,
}

#[derive(Clone, Copy)]
struct ValueRange {
    min: u32,
    max: u32,
}

impl ValueRange {
    fn includes(&self, other: u32) -> bool {
        self.min <= other && self.max >= other
    }
}
pub fn solve() {
    println!("Day 16");

    let mut input_lines = adventlib::read_input_lines("day16input.txt");

    let mut rules = Vec::new();

    while input_lines[0] != "" {
        rules.push(parse_rule(&input_lines.remove(0)))
    }

    // skip "my ticket" and headers
    input_lines.drain(0..5);

    let all_ranges: Vec<_> = rules.iter().flat_map(|rule| &rule.ranges).collect();
    let sum_invalid: u32 = input_lines
        .iter()
        .flat_map(|line| line.split(",").map(|num| num.parse::<u32>().unwrap()))
        .filter(|&num| all_ranges.iter().all(|range| !range.includes(num)))
        .sum();

    println!("Output (part 1): {}", sum_invalid);

    println!("Output (part 2): {}", -1);
}

fn parse_rule(line: &str) -> FieldRule {
    lazy_static! {
        //Example: "departure station: 30-152 or 160-959"
        static ref RULE_PATTERN: Regex = Regex::new(r"(?P<fieldName>.*): (?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)")
        .expect("Problem in pattern");
    }

    let captures = RULE_PATTERN
        .captures(line)
        .expect("line must match rule pattern");
    let ranges = vec![
        ValueRange {
            min: captures.name("min1").unwrap().as_str().parse().unwrap(),
            max: captures.name("max1").unwrap().as_str().parse().unwrap(),
        },
        ValueRange {
            min: captures.name("min2").unwrap().as_str().parse().unwrap(),
            max: captures.name("max2").unwrap().as_str().parse().unwrap(),
        },
    ];
    return FieldRule {
        field_name: captures.name("fieldName").unwrap().as_str().to_string(),
        ranges: ranges,
    };
}

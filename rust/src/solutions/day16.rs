use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

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

    //skipping blank line and header
    let my_ticket = parse_ticket(&input_lines[2]);

    // skip ahead to other ticket lines
    input_lines.drain(0..5);

    let all_ranges: Vec<_> = rules.iter().flat_map(|rule| &rule.ranges).collect();
    let sum_invalid: u32 = input_lines
        .iter()
        .flat_map(|line| parse_ticket(line))
        .filter(|&num| all_ranges.iter().all(|range| !range.includes(num)))
        .sum();

    println!("Output (part 1): {}", sum_invalid);

    let mut field_positions: HashMap<usize, HashSet<usize>> = HashMap::new();
    for field_index in 0..rules.len() {
        field_positions.insert(field_index, (0..my_ticket.len()).collect());
    }

    for line in &input_lines {
        let ticket = parse_ticket(line);

        if ticket
            .iter()
            .any(|&num| all_ranges.iter().all(|range| !range.includes(num)))
        {
            continue; // ticket is excluded as invalid
        }

        // Eliminate invalid ones.
        for (val_index, value) in ticket.iter().enumerate() {
            for (field_index, field) in rules.iter().enumerate() {
                if field.ranges.iter().all(|range| !range.includes(*value)) {
                    field_positions.entry(field_index).and_modify(|set| {
                        set.remove(&val_index);
                    });
                }
            }
        }
    }

    let mut field_indexes_to_find: HashSet<_> = (0..rules.len()).collect();
    while field_indexes_to_find.len() > 0 {
        let mut mappable_index = 999;
        for field_index in &field_indexes_to_find {
            if field_positions.get(field_index).unwrap().len() == 1 {
                mappable_index = *field_index;
                break;
            }
        }
        if mappable_index != 999 {
            let value_index = field_positions
                .get(&mappable_index)
                .unwrap()
                .iter()
                .next()
                .cloned()
                .unwrap();

            field_indexes_to_find.remove(&mappable_index);

            for (_k, set) in field_positions
                .iter_mut()
                .filter(|(&k, _v)| k != mappable_index)
            {
                set.remove(&value_index);
            }
        } else {
            panic!("We didn't make progress. Need more logic.")
        }
    }
    //println!("{:#?}", field_positions);

    let product_of_departures: u64 = rules
        .iter()
        .enumerate()
        .filter_map(|(i, rule)| {
            if rule.field_name.starts_with("departure") {
                Some(i)
            } else {
                None
            }
        })
        .filter_map(|field_i| field_positions.get(&field_i).unwrap().iter().next())
        .map(|&value_i| my_ticket[value_i] as u64)
        .product();

    println!("Output (part 2): {}", product_of_departures);
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

fn parse_ticket(line: &str) -> Vec<u32> {
    line.split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .collect()
}

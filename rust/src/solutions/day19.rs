use lazy_static::lazy_static;
use std::collections::HashMap;

use regex::Regex;

struct Rule {
    num: u32,
    match_list: Vec<Match>,
}

struct Match {
    sequence: Vec<u32>,
    string: Option<String>,
}

pub fn solve() {
    println!("Day 19");

    let input_lines = adventlib::read_input_lines("day19input.txt");

    let mut lines_to_check: Vec<_> = Vec::new();

    let mut rules: HashMap<u32, Vec<Match>> = HashMap::new();

    for line in &input_lines {
        if line == "" {
            continue;
        }
        let first_char = line.chars().next().unwrap();
        if first_char >= '0' && first_char <= '9' {
            let new_rule = parse_rule(line);
            rules.insert(new_rule.num, new_rule.match_list);
        }

        if first_char == 'a' || first_char == 'b' {
            lines_to_check.push(line.to_string())
        }
    }

    let matched_count = lines_to_check
        .iter()
        .filter(|line| match_line(line, 0, &rules))
        .count();

    println!("Output (part 1): {}", matched_count);

    // Replace two rules
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    rules.insert(
        8,
        vec![
            Match {
                sequence: vec![42],
                string: None,
            },
            Match {
                sequence: vec![42, 8],
                string: None,
            },
        ],
    );
    rules.insert(
        11,
        vec![
            Match {
                sequence: vec![42, 31],
                string: None,
            },
            Match {
                sequence: vec![42, 11, 31],
                string: None,
            },
        ],
    );

    let matched_count = lines_to_check
        .iter()
        .filter(|line| match_line(line, 0, &rules))
        .count();

    println!("Output (part 2): {}", matched_count);
}

fn parse_rule(line: &str) -> Rule {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(\d+):(.*)$").unwrap();
    }
    let captures = PATTERN.captures(line).unwrap();
    return Rule {
        num: captures[1].parse().unwrap(),
        match_list: parse_match(&captures[2]),
    };
}

fn parse_match(mut line: &str) -> Vec<Match> {
    line = line.trim();
    if line.contains("\"") {
        return vec![Match {
            sequence: vec![],
            string: Some(line.chars().nth(1).unwrap().to_string()),
        }];
    } else {
        let branches = line
            .split(" | ")
            .map(|seq| Match {
                sequence: seq
                    .split_ascii_whitespace()
                    .map(|r| r.parse::<u32>().unwrap())
                    .collect(),
                string: None,
            })
            .collect();
        return branches;
    }
}

fn match_line(line: &str, start_rule: u32, rule_tree: &HashMap<u32, Vec<Match>>) -> bool {
    let mut left_to_match = Vec::new();
    left_to_match.push(start_rule);
    is_match(line, &mut left_to_match, rule_tree)
}

fn is_match(
    line: &str,
    left_to_match: &mut Vec<u32>,
    rule_tree: &HashMap<u32, Vec<Match>>,
) -> bool {
    if left_to_match.len() > line.len() {
        return false;
    }

    if left_to_match.len() == 0 && line.len() == 0 {
        return true;
    } else if left_to_match.len() == 0 {
        return false;
    }

    let next_rule_num = left_to_match.pop().unwrap();
    let new_matches = rule_tree.get(&next_rule_num).unwrap();
    for branch in new_matches {
        match &branch.string {
            Some(literal) => {
                if line.starts_with(literal)
                    && is_match(&line[literal.len()..], left_to_match, rule_tree)
                {
                    return true;
                }
            }
            None => {
                let expand_size = branch.sequence.len();
                left_to_match.extend(branch.sequence.iter().cloned().rev());
                if is_match(line, left_to_match, rule_tree) {
                    return true;
                } else {
                    for _ in 0..expand_size {
                        left_to_match.pop();
                    }
                }
            }
        }
    }

    left_to_match.push(next_rule_num);

    return false;
}

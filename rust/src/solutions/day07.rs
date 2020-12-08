use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::repeat;

struct BagRule {
    color: String,
    contents: Vec<(u32, String)>,
}

pub fn solve() {
    println!("Day 7");

    let input = adventlib::read_input_lines("day07input.txt");
    let rules: Vec<_> = input.iter().map(|l| parse_rule(l)).collect();
    let start = "shiny gold";

    let mut inverted_rules: HashMap<&str, HashSet<String>> = HashMap::new();

    let inverted_pairs = rules.iter().flat_map(|rule| {
        rule.contents
            .iter()
            .map(|(_, col)| col)
            .zip(repeat(rule.color.clone()))
    });
    for pair in inverted_pairs {
        inverted_rules
            .entry(pair.0)
            .and_modify(|set| {
                set.insert(pair.1.clone());
            })
            .or_insert([pair.1].iter().cloned().collect());
    }

    let mut visited: HashSet<String> = HashSet::new();

    let answer = dfs_count(start, &inverted_rules, &mut visited);

    println!("Output (part 1): {}", answer);

    let mut forward_rules: HashMap<&str, _> = HashMap::new();

    for rule in rules.iter() {
        let rule_children: HashSet<(u32, String)> = rule.contents.iter().cloned().collect();
        forward_rules.insert(&rule.color, rule_children);
    }

    // Should really be doing dynamic programming
    let answer = dfs_count_to_leaves(start, &forward_rules);

    println!("Output (part 2): {}", answer);
}

fn parse_rule(line: &str) -> BagRule {
    lazy_static! {
        static ref CONTENTS_RE: Regex =
            Regex::new(r"(?P<count>\d+) (?P<color>\w+ \w+) bag").expect("Invalid rule pattern");
    }
    let color = line
        .split_ascii_whitespace()
        .take(2)
        .collect::<Vec<_>>()
        .join(" ");
    let content_colors = CONTENTS_RE.captures_iter(line).filter_map(|cap| {
        match (cap.name("count"), cap.name("color")) {
            (Some(count), Some(color)) => Some((
                count.as_str().parse::<u32>().unwrap(),
                color.as_str().to_string(),
            )),
            _ => None,
        }
    });
    BagRule {
        color: color,
        contents: content_colors.collect(),
    }
}

fn dfs_count(
    start: &str,
    edges: &HashMap<&str, HashSet<String>>,
    visited: &mut HashSet<String>,
) -> u32 {
    let mut count = 0;
    for next in edges[start].iter() {
        count += dfs_count_rec(next, edges, visited);
    }
    return count;
}

fn dfs_count_rec(
    start: &str,
    edges: &HashMap<&str, HashSet<String>>,
    visited: &mut HashSet<String>,
) -> u32 {
    if visited.contains(start) {
        return 0;
    }

    visited.insert(start.to_owned());
    let mut count = 1;

    if !edges.contains_key(start) {
        return count;
    }

    for next in edges[start].iter() {
        count += dfs_count_rec(next, edges, visited);
    }
    return count;
}

fn dfs_count_to_leaves(start: &str, edges: &HashMap<&str, HashSet<(u32, String)>>) -> u32 {
    let mut total_count = 0;

    if !edges.contains_key(start) {
        return total_count;
    }

    for (count, child) in edges[start].iter() {
        total_count += count + count * dfs_count_to_leaves(&child, edges);
    }
    return total_count;
}

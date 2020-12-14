use regex::Regex;
use std::collections::HashMap;

pub fn solve() {
    println!("Day 14");

    let input_lines = adventlib::read_input_lines("day14input.txt");

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut or_mask = 0;
    let mut and_mask = 0;

    for line in &input_lines {
        if line.starts_with("mask") {
            or_mask = 0;
            and_mask = 0;
            let raw_mask = &line["mask = ".len()..];
            for (i, ch) in raw_mask.chars().rev().enumerate() {
                let cur_bit = 1 << i;
                match ch {
                    '0' => (),
                    '1' => {
                        or_mask |= cur_bit;
                        and_mask |= cur_bit;
                    }
                    _ => and_mask |= cur_bit,
                }
            }
        } else {
            let captures = Regex::new(r"mem\[(\d+)\] = (\d+)")
                .expect("mem pattern")
                .captures(line)
                .unwrap();
            let addr: u64 = captures[1].parse().unwrap();
            let arg: u64 = captures[2].parse().unwrap();

            mem.insert(addr, (arg & and_mask) | or_mask);
        }
    }

    let answer: u64 = mem.values().sum();
    println!("Output (part 1): {}", answer);

    println!("Output (part 2): {}", -1);
}

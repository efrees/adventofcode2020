use regex::Regex;
use std::collections::HashMap;

struct Assignment {
    addr: u64,
    arg: u64,
}

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
            let instr = parse_assignment(line);
            mem.insert(instr.addr, (instr.arg & and_mask) | or_mask);
        }
    }

    let answer: u64 = mem.values().sum();
    println!("Output (part 1): {}", answer);

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut raw_mask: &str = "";

    for line in &input_lines {
        if line.starts_with("mask") {
            raw_mask = &line["mask = ".len()..];
        } else {
            let instr = parse_assignment(line);
            insert_all(&mut mem, &instr, &raw_mask);
        }
    }

    let answer: u64 = mem.values().sum();
    println!("Output (part 2): {}", answer);
}

fn parse_assignment(line: &str) -> Assignment {
    let captures = Regex::new(r"mem\[(\d+)\] = (\d+)")
        .expect("mem pattern")
        .captures(line)
        .unwrap();
    Assignment {
        addr: captures[1].parse().unwrap(),
        arg: captures[2].parse().unwrap(),
    }
}

fn insert_all(mem: &mut HashMap<u64, u64>, instr: &Assignment, raw_mask: &str) {
    let x_count = raw_mask.chars().filter(|&c| c == 'X').count();
    for floating in 0..(1 << x_count) {
        let mut floating_i = 0;
        let mut addr_value = 0u64;
        for (output_i, ch) in (0..36).rev().zip(raw_mask.chars()) {
            let next_bit = match ch {
                '0' => (instr.addr >> output_i) & 1,
                '1' => 1,
                'X' => {
                    let bit = (floating >> floating_i) & 1;
                    floating_i += 1;
                    bit
                }
                _ => panic!("Unexpected character in mask '{}'", ch),
            };
            addr_value <<= 1;
            addr_value += next_bit;
        }
        mem.insert(addr_value, instr.arg);
    }
}

use std::iter::repeat;

pub fn solve() {
    println!("Day 10");

    let input_lines = adventlib::read_input_lines("day10input.txt");

    let mut parsed_lines: Vec<_> = input_lines
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    parsed_lines.sort();

    // add device itself
    parsed_lines.push(parsed_lines.last().unwrap() + 3);

    let mut one_count = 0;
    let mut three_count = 0;
    let mut last_joltage = 0;
    for &joltage in parsed_lines.iter() {
        match joltage - last_joltage {
            1 => one_count += 1,
            3 => three_count += 1,
            _ => (),
        }
        last_joltage = joltage;
    }

    println!("Output (part 1): {}", one_count * three_count);

    // add port for simplicity
    parsed_lines.insert(0, 0);

    let length = parsed_lines.len();
    let mut partial_answers: Vec<u64> = repeat(0).take(length).collect();
    partial_answers[0] = 1;

    for i in 0..length {
        let joltage = parsed_lines[i];
        let mut j = i + 1;
        while j < length && parsed_lines[j] <= joltage + 3 {
            partial_answers[j] += partial_answers[i];
            j += 1;
        }
    }

    println!("Output (part 2): {}", partial_answers.last().unwrap());
}

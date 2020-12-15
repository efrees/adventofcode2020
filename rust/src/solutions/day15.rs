use std::collections::HashMap;

pub fn solve() {
    println!("Day 15");

    let input = [1u32, 20, 11, 6, 12, 0];
    let mut last_said_on: HashMap<u32, u32> = HashMap::new();
    let mut last_said: u32 = 0;
    let mut previously_said_on = 0;

    for (i, num) in input.iter().enumerate() {
        last_said = *num;
        match last_said_on.insert(*num, i as u32 + 1) {
            Some(previous) => previously_said_on = previous,
            None => previously_said_on = 0,
        }
    }

    let previous_turns = input.len() as u32;
    for turn in previous_turns + 1..=2020 {
        if previously_said_on == 0 {
            last_said = 0;
        } else {
            last_said = turn - 1 - previously_said_on;
        }
        match last_said_on.insert(last_said, turn) {
            Some(previous) => previously_said_on = previous,
            None => previously_said_on = 0,
        }
    }

    let answer = last_said;
    println!("Output (part 1): {}", answer);

    let previous_turns = 2020;
    for turn in previous_turns + 1..=30000000 {
        if previously_said_on == 0 {
            last_said = 0;
        } else {
            last_said = turn - 1 - previously_said_on;
        }
        match last_said_on.insert(last_said, turn) {
            Some(previous) => previously_said_on = previous,
            None => previously_said_on = 0,
        }
    }

    let answer = last_said;
    println!("Output (part 2): {}", answer);
}

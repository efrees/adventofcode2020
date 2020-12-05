use std::collections::HashSet;

pub fn solve() {
    println!("Day 5");

    let input_lines = adventlib::read_input_lines("day05input.txt");

    let seat_ids = input_lines.iter().map(|line| get_seat_id(line));

    println!("Output (part 1): {}", seat_ids.clone().max().unwrap());

    let taken_seat_ids: HashSet<_> = seat_ids.collect();
    let my_seat = (1u32..128 * 8)
        .filter(|id| {
            !taken_seat_ids.contains(id)
                && taken_seat_ids.contains(&(id - 1))
                && taken_seat_ids.contains(&(id + 1))
        })
        .next();

    println!("Output (part 2): {}", my_seat.unwrap());
}

fn get_seat_id(directions: &str) -> u32 {
    to_binary(&directions)
}

fn to_binary(directions: &str) -> u32 {
    let mut value = 0;
    for c in directions.chars() {
        value = match c {
            'F' | 'L' => value * 2,
            'B' | 'R' => value * 2 + 1,
            _ => value,
        }
    }
    return value;
}

use adventlib::grid::*;

pub fn solve() {
    println!("Day 12");

    let input_lines = adventlib::read_input_lines("day12input.txt");

    let parsed_lines: Vec<_> = input_lines
        .iter()
        .map(|line| parse_instruction(line))
        .collect();

    let mut position = Point::origin();
    let mut forward = Direction::Right;

    for (dir, dist) in parsed_lines {
        let new_forward = match dir {
            'L' => forward.turn_left_times((dist / 90) as usize),
            'R' => forward.turn_right_times((dist / 90) as usize),
            _ => forward,
        };
        let move_vec = match dir {
            'N' => Direction::Up.as_vector(),
            'S' => Direction::Down.as_vector(),
            'E' => Direction::Right.as_vector(),
            'W' => Direction::Left.as_vector(),
            'F' => forward.as_vector(),
            'L' | 'R' | _ => Point::origin(),
        };

        position = position.vec_add(&move_vec.vec_scale(dist as i64));
        forward = new_forward;
    }

    println!("Output (part 1): {}", position.manhattan_dist_to(&Point::origin()));

    println!("Output (part 2): {}", 0);
}

fn parse_instruction(line: &str) -> (char, u32) {
    let dir = line.chars().nth(0).expect("no empty line");
    let dist = line[1..]
        .parse()
        .expect("line must have a valid integer after the first character");
    return (dir, dist);
}

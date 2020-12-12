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

    for (dir, dist) in parsed_lines.iter() {
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

        position = position.vec_add(&move_vec.vec_scale(*dist as i64));
        forward = new_forward;
    }

    println!(
        "Output (part 1): {}",
        position.manhattan_dist_to(&Point::origin())
    );

    let mut position = Point::origin();
    // Sticking with "up" == -y, as in most of the grid problems
    let mut waypoint = Point::new(10, -1);

    for (dir, dist) in parsed_lines.iter() {
        let magnitude = *dist as i64;
        if dir == &'F' {
            position = position.vec_add(&waypoint.vec_scale(magnitude));
        } else {
            waypoint = match dir {
                'N' | 'S' | 'E' | 'W' => waypoint.vec_add(
                    &Direction::from_char(*dir)
                        .unwrap()
                        .as_vector()
                        .vec_scale(magnitude),
                ),
                'L' => rotate_waypoint_right(&waypoint, 360 - magnitude),
                'R' => rotate_waypoint_right(&waypoint, magnitude),
                _ => waypoint,
            };
        }
    }

    println!(
        "Output (part 2): {}",
        position.manhattan_dist_to(&Point::origin())
    );
}

fn parse_instruction(line: &str) -> (char, u32) {
    let dir = line.chars().nth(0).expect("no empty line");
    let dist = line[1..]
        .parse()
        .expect("line must have a valid integer after the first character");
    return (dir, dist);
}

fn rotate_waypoint_right(waypoint: &Point, degrees: i64) -> Point {
    // Since "Up" is toward -y, "right" and "left" are swapped
    // relative to "normal" math.
    match degrees / 90 {
        0 => *waypoint,
        1 => Point::new(-waypoint.y, waypoint.x),
        2 => waypoint.vec_scale(-1),
        3 => Point::new(waypoint.y, -waypoint.x),
        _ => panic!("Write a real calculation!"),
    }
}

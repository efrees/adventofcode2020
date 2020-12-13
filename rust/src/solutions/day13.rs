pub fn solve() {
    println!("Day 13");

    let input_lines = adventlib::read_input_lines("day13input.txt");

    let earliest_time: u32 = input_lines[0].parse().unwrap();
    let busses: Vec<_> = input_lines[1]
        .split(',')
        .filter_map(|line| line.parse::<u32>().ok())
        .collect();

    let first_bus = busses
        .iter()
        .map(|&bus| (bus, bus - (earliest_time % bus)))
        .min_by_key(|&(_, wait_time)| wait_time)
        .unwrap();

    let answer = first_bus.0 * first_bus.1;
    println!("Output (part 1): {}", answer);

    println!("Output (part 2): {}", -1);
}

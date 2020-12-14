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

    let offset_busses: Vec<_> = input_lines[1]
        .split(',')
        .enumerate()
        .filter_map(|(i, line)| line.parse::<u64>().ok().map(|bus| (i, bus)))
        .collect();

    // Since all bus ids in the input are prime, all are generators of the
    // group of integers modulo n, for all n that isn't a multiple of the id.
    // This means if we find the smallest multiple, k, of the id for that bus, b,
    // to have the desired offset relative to some other bus a, the solution will
    // repeat every b*a. The solution set will be C + n*b*a

    // The key idea here is preserving all previously solved conditions by only
    // adding multiples of the GCD of solved busses to the offset.

    let mut offset = 0;
    let mut period = 1;

    for (target_offset, bus_id) in &offset_busses {
        let closest_offset = *target_offset as u64 % bus_id;
        if offset % bus_id != closest_offset {
            while offset % bus_id != bus_id - closest_offset {
                offset += period
            }
        }
        period *= bus_id;
    }

    println!("Output (part 2): {}", offset);
}

pub fn solve() {
    println!("Day 3");

    let input_lines = adventlib::read_input_lines("day03input.txt");

    let input_width = input_lines
        .first()
        .map(|line| line.len())
        .expect("We must have input");

    let tree_count = count_on_slope((3, 1), &input_lines, input_width);

    println!("Output (part 1): {}", tree_count);

    let all_slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product = 1;
    for slope in all_slopes {
        product *= count_on_slope(slope, &input_lines, input_width);
    }

    println!("Output (part 2): {}", product);
}

fn count_on_slope(slope: (usize, usize), map: &Vec<String>, map_width: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let (dx, dy) = slope;
    let mut tree_count = 0;

    while y < map.len() {
        match map.get(y).and_then(|line| line.chars().nth(x % map_width)) {
            Some('#') => tree_count += 1,
            _ => (),
        };

        x += dx;
        y += dy;
    }

    return tree_count;
}

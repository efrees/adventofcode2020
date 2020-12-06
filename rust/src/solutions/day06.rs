use std::collections::HashSet;

pub fn solve() {
    println!("Day 6");

    let input = adventlib::read_input_raw("day06input.txt");

    let groups = input.split("\n\n");

    let mut total_count = 0;
    for group in groups.clone() {
        let char_set: HashSet<_> = group.chars().filter(|&x| x != '\n').collect();
        total_count += char_set.len();
    }

    println!("Output (part 1): {}", total_count);

    let mut total_count = 0;
    for group in groups.clone() {
        let members = group.split("\n");
        let mut char_sets: Vec<HashSet<_>> = members
            .filter(|s| s.len() > 0)
            .map(|m| m.chars().collect::<HashSet<_>>())
            .collect();

        let mut char_set = char_sets.remove(0);
        for set in char_sets {
            char_set = char_set.intersection(&set).cloned().collect();
        }
        total_count += char_set.len();
    }

    println!("Output (part 2): {}", total_count);
}

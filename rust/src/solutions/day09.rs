pub fn solve() {
    println!("Day 9");

    let input_lines = adventlib::read_input_lines("day09input.txt");

    let parsed_lines: Vec<_> = input_lines
        .iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let mut answer_one = 0u64;
    for i in 25..parsed_lines.len() - 1 {
        if !has_pair_with_sum(&parsed_lines[i - 25..i], parsed_lines[i]) {
            answer_one = parsed_lines[i];
            break;
        }
    }

    println!("Output (part 1): {}", answer_one);

    let answer_two = find_contiguous_set_with_sum(&parsed_lines, answer_one);

    println!("Output (part 2): {}", answer_two);
}

fn has_pair_with_sum(list: &[u64], target: u64) -> bool {
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            if list[i] + list[j] == target {
                return true;
            }
        }
    }
    return false;
}

fn find_contiguous_set_with_sum(list: &Vec<u64>, target: u64) -> u64 {
    let mut low = 0;
    let mut high = low + 1;
    let mut sum = list[low] + list[high];

    while sum != target {
        if sum < target || high == low + 1 {
            high += 1;
            if high == list.len() {
                return 0;
            }
            sum += list[high];
        }

        if sum > target {
            sum -= list[low];
            low += 1;
        }
    }

    let min = list[low..=high].iter().min().unwrap();
    let max = list[low..=high].iter().max().unwrap();
    return min + max;
}

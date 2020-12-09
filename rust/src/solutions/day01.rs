pub fn solve() {
    println!("Day 1");

    let input_lines = adventlib::read_input_lines("day01input.txt");

    let mut parsed_lines: Vec<_> = input_lines
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    parsed_lines.sort();

    let answer = find_pair_with_sum(parsed_lines.as_slice(), 2020);

    println!("Output (part 1): {}", answer);

    let mut answer = 0;
    for low in 0..parsed_lines.len() - 2 {
        let target = 2020 - parsed_lines[low];

        let partial_answer = find_pair_with_sum(&parsed_lines[low + 1..], target);

        if partial_answer > 0 {
            answer = parsed_lines[low] * partial_answer;
            break;
        }
    }

    println!("Output (part 2): {}", answer);
}

fn find_pair_with_sum(sorted_list: &[u32], target: u32) -> u32 {
    let mut low = 0;
    let mut high = sorted_list.len() - 1;
    let mut answer = 0;

    while low < high && answer == 0 {
        let a = sorted_list[low];
        let b = sorted_list[high];

        match a + b {
            sum if sum == target => answer = a * b,
            sum if sum < target => low += 1,
            sum if sum > target => high -= 1,
            _ => (),
        }
    }

    return answer;
}

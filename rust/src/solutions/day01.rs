pub fn solve() {
    println!("Day 1");

    let input_lines = adventlib::read_input_lines("day01input.txt");

    let mut parsed_lines: Vec<_> = input_lines
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    parsed_lines.sort();

    let mut low = 0;
    let mut high = parsed_lines.len() - 1;
    let mut answer = 0;

    while low < high && answer == 0 {
        let a = parsed_lines[low];
        let b = parsed_lines[high];

        match a + b {
            2020 => answer = a * b,
            sum if sum < 2020 => low += 1,
            sum if sum > 2020 => high -= 1,
            _ => (),
        }
    }

    println!("Output (part 1): {}", answer);

    let mut low = 0;
    let mut high = parsed_lines.len() - 1;
    let mut answer = 0;

    while low < high && answer == 0 {
        let a = parsed_lines[low];
        let b = parsed_lines[high];
        let sum = a + b;

        if sum + parsed_lines[low + 1] > 2020 {
            high -= 1;
            continue;
        }
        if sum + parsed_lines[high - 1] < 2020 {
            // Can't get large enough
            low += 1;
            continue;
        }

        let target = 2020 - sum;
        match parsed_lines.as_slice()[low + 1..high].binary_search(&target) {
            Ok(_) => answer = a * b * target,
            _ => (),
        }

        low += 1;
    }

    println!("Output (part 2): {}", answer);
}

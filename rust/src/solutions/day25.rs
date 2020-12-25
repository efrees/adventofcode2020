pub fn solve() {
    println!("Day 25");

    let input_lines = adventlib::read_input_lines("day25input.txt");

    let public_keys: Vec<_> = input_lines
        .iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let subject_number = 7;
    let modulo = 20201227;
    let mut current_number = 1;
    let mut cur_loop_count = 0;
    let mut secret_count_one = None;
    let mut secret_count_two = None;
    while secret_count_one.is_none() || secret_count_two.is_none() {
        current_number *= subject_number;
        current_number %= modulo;
        cur_loop_count += 1;

        if current_number == public_keys[0] && secret_count_one == None {
            secret_count_one = Some(cur_loop_count);
        }

        if current_number == public_keys[1] && secret_count_two == None {
            secret_count_two = Some(cur_loop_count);
        }
    }

    let subject_number = public_keys[0];
    let loop_count = secret_count_two.unwrap();
    current_number = 1;
    for _ in 0..loop_count {
        current_number *= subject_number;
        current_number %= modulo;
    }

    println!("Output (part 1): {}", current_number);
}

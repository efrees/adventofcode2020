use adventlib::collections::{CircleList, CircleListPointer};

pub fn solve() {
    println!("Day 23");

    let input = adventlib::read_input_raw("day23input.txt");

    let mut cup_order: Vec<_> = input.trim().as_bytes().iter().map(|c| c - b'0').collect();

    // We'll rotate to ensure each round starts with the current cup at the beginning.
    for _ in 0..100 {
        let current = cup_order[0];
        let next_three: Vec<_> = cup_order[1..].iter().take(3).cloned().collect();

        cup_order.drain(0..4);
        cup_order.push(current);

        let mut target = if current > 1 { current - 1 } else { 9 };

        while next_three.contains(&target) {
            target = if target > 1 { target - 1 } else { 9 }
        }

        let target_i = cup_order
            .iter()
            .position(|&x| x == target)
            .expect("Check your target assumptions");

        for cup in next_three.iter().cloned().rev() {
            cup_order.insert(target_i + 1, cup);
        }
    }

    let one_position = cup_order
        .iter()
        .position(|&x| x == 1)
        .expect("Don't lose the one");
    cup_order.rotate_left(one_position);

    let output: String = cup_order[1..].iter().map(|b| b.to_string()).collect();
    println!("Output (part 1): {}", output);

    println!("Output (part 2): {}", -1);
}

fn print_circle_from(start: CircleListPointer, list: &CircleList<u32>) {
    let mut v = Vec::new();
    let mut next = list.next_node(start);
    while start != next {
        v.push(list.get_value(next).unwrap());
        next = list.next_node(next);
    }
    println!("{:?}", v);
}

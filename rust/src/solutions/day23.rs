use std::collections::VecDeque;

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

    let max_value: u32 = 1_000_000;
    let mut cup_order: VecDeque<u32> = input
        .trim()
        .as_bytes()
        .iter()
        .map(|c| (c - b'0') as u32)
        .chain(10..)
        .take(max_value as usize)
        .collect();

    // We'll rotate to ensure each round starts with the current cup at the beginning.
    for i in 0..10_000_000 {
        if i % 1000 == 0 {
            println!("making progress {}", i);
        }
        let current = cup_order[0];
        let next_three: Vec<_> = vec![cup_order[1], cup_order[2], cup_order[3]];

        cup_order.drain(0..4);
        cup_order.push_back(current);

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
    let next1 = cup_order[one_position + 1];
    let next2 = cup_order[one_position + 2];

    let output = next1 as u64 * next2 as u64;

    println!("Output (part 2): {}", output);
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

use std::collections::HashMap;

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
    let mut cup_order: CircleList<u32> = CircleList::with_capacity(max_value as usize);
    let mut cup_pointers = HashMap::new();

    for cup in input
        .trim()
        .as_bytes()
        .iter()
        .map(|c| (c - b'0') as u32)
        .chain(10..)
        .take(max_value as usize)
    {
        cup_order.insert(cup);
        cup_pointers.insert(cup, cup_order.last.unwrap());
    }

    let mut current = cup_order.next_node(cup_order.last.unwrap()); // wrap to first;

    // We'll rotate to ensure each round starts with the current cup at the beginning.
    for _ in 0..10_000_000 {
        let current_value = cup_order.get_value(current).unwrap();
        let mut next_three = Vec::new();
        let mut next = cup_order.next_node(current);

        for _ in 0..3 {
            next_three.push(cup_order.get_value(next).unwrap());
            next = cup_order.next_node(next);
        }

        let mut target = if current_value > 1 {
            current_value - 1
        } else {
            max_value
        };

        while next_three.contains(&target) {
            target = if target > 1 { target - 1 } else { max_value }
        }

        let destination_node = cup_pointers
            .get(&target)
            .expect("Index should have all targets");

        for cup in next_three.iter().rev() {
            let moving_node = cup_pointers
                .get(cup)
                .expect("Index should have all moved cups");
            cup_order.move_node_after(*moving_node, *destination_node);
        }

        current = next;
    }

    let mut one_node = current;
    while cup_order.get_value(one_node) != Some(1) {
        one_node = cup_order.next_node(one_node);
    }

    let next1 = cup_order.next_node(one_node);
    let next2 = cup_order.next_node(next1);

    let output =
        cup_order.get_value(next1).unwrap() as u64 * cup_order.get_value(next2).unwrap() as u64;

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

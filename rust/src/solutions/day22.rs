use std::collections::HashSet;

pub fn solve() {
    println!("Day 22");

    let input_lines = adventlib::read_input_lines("day22input.txt");

    let (mut deck1, mut deck2) = parse_decks(&input_lines);

    while deck1.len() * deck2.len() > 0 {
        let top1 = deck1.remove(0);
        let top2 = deck2.remove(0);

        if top1 > top2 {
            deck1.push(top1);
            deck1.push(top2);
        } else {
            deck2.push(top2);
            deck2.push(top1);
        }
    }

    let winning_deck = if deck1.len() > 0 { deck1 } else { deck2 };
    let score = score_deck(&winning_deck);

    println!("Output (part 1): {}", score);

    let (mut deck1, mut deck2) = parse_decks(&input_lines);

    let winner = play_recursive(&mut deck1, &mut deck2);
    let winning_deck = if winner == 1 { deck1 } else { deck2 };
    let score = score_deck(&winning_deck);

    println!("Output (part 2): {}", score);
}

fn parse_decks(lines: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut deck1 = Vec::new();
    let mut deck2 = Vec::new();

    let mut reading_one = true;
    for line in lines {
        if line.starts_with("Player") {
            continue;
        }

        if line == "" {
            reading_one = false;
            continue;
        }

        let value: u32 = line.parse().unwrap();

        if reading_one {
            deck1.push(value);
        } else {
            deck2.push(value);
        }
    }

    (deck1, deck2)
}

fn score_deck(deck: &Vec<u32>) -> u32 {
    deck.iter()
        .rev()
        .zip(1u32..)
        .map(|(card, val)| card * val)
        .sum()
}

fn play_recursive(deck1: &mut Vec<u32>, deck2: &mut Vec<u32>) -> u8 {
    let mut seen_states: HashSet<_> = HashSet::new();

    while deck1.len() * deck2.len() > 0 {
        let game_state = get_game_state(&deck1, &deck2);
        if seen_states.contains(&game_state) {
            return 1;
        }
        seen_states.insert(game_state);

        let top1 = deck1.remove(0);
        let top2 = deck2.remove(0);

        let winner = if top1 as usize <= deck1.len() && top2 as usize <= deck2.len() {
            let mut deck1_copy: Vec<_> = deck1.iter().cloned().take(top1 as usize).collect();
            let mut deck2_copy: Vec<_> = deck2.iter().cloned().take(top2 as usize).collect();
            play_recursive(&mut deck1_copy, &mut deck2_copy)
        } else if top1 > top2 {
            1
        } else {
            2
        };

        if winner == 1 {
            deck1.push(top1);
            deck1.push(top2);
        } else {
            deck2.push(top2);
            deck2.push(top1);
        }
    }

    return if deck1.len() > 0 { 1 } else { 2 };
}

fn get_game_state(deck1: &Vec<u32>, deck2: &Vec<u32>) -> String {
    let deck1_strings: Vec<_> = deck1.iter().map(|v| v.to_string()).collect();
    let deck2_strings: Vec<_> = deck2.iter().map(|v| v.to_string()).collect();
    [deck1_strings.join(""), deck2_strings.join("")].join("|")
}

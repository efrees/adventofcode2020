pub fn solve() {
    println!("Day 18");

    let input_lines = adventlib::read_input_lines("day18input.txt");

    let mut sum = 0;

    for line in input_lines {
        sum += calculate_expression(&line);
    }

    println!("Output (part 1): {}", sum);

    println!("Output (part 2): {}", -1);
}

fn calculate_expression(expression: &str) -> u64 {
    let normalized_expression = expression.replace('(', "( ").replace(')', " )");
    let mut tokens: Vec<_> = normalized_expression.split_ascii_whitespace().collect();
    return calculate_from_tokens(&mut tokens);
}

fn calculate_from_tokens(tokens: &mut Vec<&str>) -> u64 {
    let mut value = 0;
    let mut cur_operator = "+";
    while tokens.len() > 0 {
        let token = tokens.remove(0);
        if token == ")" {
            return value;
        }

        if token == "(" {
            let next_value = calculate_from_tokens(tokens);
            value = combine_terms(value, next_value, cur_operator);
        } else if token == "*" || token == "+" {
            cur_operator = token;
        } else if is_number(token) {
            let next_value = parse_number(token);
            value = combine_terms(value, next_value, cur_operator);
        }
    }
    return value;
}

fn combine_terms(p1: u64, p2: u64, op: &str) -> u64 {
    match op {
        "*" => p1 * p2,
        "+" => p1 + p2,
        _ => panic!("Unrecognized operator {}", op),
    }
}

fn is_number(token: &str) -> bool {
    match token.chars().next() {
        Some('0'..='9') => true,
        _ => false,
    }
}

fn parse_number(token: &str) -> u64 {
    token.parse().unwrap()
}

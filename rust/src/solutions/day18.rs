pub fn solve() {
    println!("Day 18");

    let input_lines = adventlib::read_input_lines("day18input.txt");

    let mut sum = 0;
    for line in &input_lines {
        sum += calculate_expression_one(&line);
    }

    println!("Output (part 1): {}", sum);

    let mut sum = 0;
    for line in &input_lines {
        sum += calculate_expression_two(&line);
    }

    println!("Output (part 2): {}", sum);
}

fn calculate_expression_one(expression: &str) -> u64 {
    let normalized_expression = expression.replace('(', "( ").replace(')', " )");
    let mut tokens: Vec<_> = normalized_expression.split_ascii_whitespace().collect();
    return calculate_from_tokens_one(&mut tokens);
}

fn calculate_from_tokens_one(tokens: &mut Vec<&str>) -> u64 {
    let mut value = 0;
    let mut cur_operator = "+";
    while tokens.len() > 0 {
        let token = tokens.remove(0);
        if token == ")" {
            return value;
        }

        if token == "(" {
            let next_value = calculate_from_tokens_one(tokens);
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

fn calculate_expression_two(expression: &str) -> u64 {
    let normalized_expression = expression.replace('(', "( ").replace(')', " )");
    let mut tokens: Vec<_> = normalized_expression.split_ascii_whitespace().collect();
    return calculate_from_tokens_two(&mut tokens);
}

fn calculate_from_tokens_two(tokens: &mut Vec<&str>) -> u64 {
    let mut token_buffer = Vec::new();
    while tokens.len() > 0 {
        let token = tokens.remove(0);
        if token == ")" {
            break;
        }

        if match token {
            "*" | "+" => true,
            _ => false,
        } {
            token_buffer.push(token.to_string());
            continue;
        }
        // Should be a number or nested expression
        let mut next_value = if token == "(" {
            calculate_from_tokens_two(tokens)
        } else if is_number(token) {
            parse_number(token)
        } else {
            panic!("Unhandled token '{}'", token);
        };

        let cur_operator = token_buffer.pop();
        match cur_operator {
            Some(op) if op == "+" => {
                let previous_value = parse_number(&token_buffer.pop().unwrap());
                next_value = combine_terms(previous_value, next_value, &op);
            }
            _ => (),
        };

        token_buffer.push(next_value.to_string());
    }

    // Only multiplications (or single terms) left on the buffer
    return token_buffer
        .iter()
        .filter(|token| is_number(token))
        .map(|token| parse_number(token))
        .fold(1, |current, next| combine_terms(current, next, "*"));
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

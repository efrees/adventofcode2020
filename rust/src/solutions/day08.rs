use std::collections::HashSet;

struct Instruction {
    op_code: String,
    arg: i32,
}

pub fn solve() {
    println!("Day 8");

    let input = adventlib::read_input_lines("day08input.txt");

    let program: Vec<_> = input.iter().map(|line| parse_instr(line)).collect();
    let mut acc = 0;
    let mut current_instr: i32 = 0;
    let mut visited_instructions = HashSet::new();

    loop {
        if current_instr < 0
            || current_instr as usize >= program.len()
            || visited_instructions.contains(&current_instr)
        {
            break;
        }

        visited_instructions.insert(current_instr);

        let instr = program
            .get(current_instr as usize)
            .expect("Invalid instruction pointer");
        match instr.op_code.as_str() {
            "acc" => acc += instr.arg,
            "jmp" => current_instr += instr.arg - 1,
            "nop" => (),
            _ => break,
        };

        current_instr += 1;
    }

    println!("Output (part 1): {}", acc);

    let program: Vec<_> = input.iter().map(|line| parse_instr(line)).collect();
    let mut acc = 0;

    for err_instr in 0..program.len() {
        acc = 0;
        let mut current_instr: i32 = 0;
        let mut visited_instructions = HashSet::new();
        let mut invalid_exit = false;

        loop {
            if current_instr < 0
                || current_instr as usize > program.len()
                || visited_instructions.contains(&current_instr)
            {
                invalid_exit = true;
                break;
            }

            if current_instr as usize == program.len() {
                break;
            }

            visited_instructions.insert(current_instr);

            let instr = program
                .get(current_instr as usize)
                .expect("Invalid instruction pointer");

            let mut op_code = instr.op_code.as_str();
            if current_instr as usize == err_instr {
                match op_code {
                    "jmp" => op_code = "nop",
                    "nop" => op_code = "jmp",
                    _ => (),
                }
            }

            match op_code {
                "acc" => acc += instr.arg,
                "jmp" => current_instr += instr.arg - 1,
                "nop" => (),
                _ => break,
            };

            current_instr += 1;
        }

        if !invalid_exit {
            break;
        }
    }

    println!("Output (part 2): {}", acc);
}

fn parse_instr(line: &str) -> Instruction {
    let parts: Vec<_> = line.split_ascii_whitespace().collect();

    Instruction {
        op_code: parts.get(0).cloned().expect("Expected opcode").to_owned(),
        arg: parts
            .get(1)
            .cloned()
            .expect("Expected arg")
            .parse()
            .unwrap(),
    }
}

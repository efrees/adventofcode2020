use adventlib::grid::{Point, SparseGrid};

pub fn solve() {
    println!("Day 24");

    let input_lines = adventlib::read_input_lines("day24input.txt");

    let mut grid: SparseGrid<char> = SparseGrid::new();

    for line in &input_lines {
        let mut chars: Vec<_> = line.chars().collect();

        let mut current = Point::new(0, 0);
        while chars.len() > 0 {
            let dir = if chars[0] == 'e' || chars[0] == 'w' {
                chars.remove(0).to_string()
            } else {
                chars.drain(0..2).collect::<String>()
            };

            let east_offset = current.y.abs() % 2;
            let west_offset = 1 - east_offset;

            if dir == "e" {
                current = Point::new(current.x + 1, current.y);
            } else if dir == "w" {
                current = Point::new(current.x - 1, current.y);
            } else if dir == "sw" {
                current = Point::new(current.x - west_offset, current.y + 1);
            } else if dir == "se" {
                current = Point::new(current.x + east_offset, current.y + 1);
            } else if dir == "nw" {
                current = Point::new(current.x - west_offset, current.y - 1);
            } else if dir == "ne" {
                current = Point::new(current.x + east_offset, current.y - 1);
            }
        }

        match grid.get(&current) {
            Some(&'B') => grid.insert(current, 'W'),
            _ => grid.insert(current, 'B'),
        }
    }

    let count_black = grid.iter().filter(|(_p, &x)| x == 'B').count();

    println!("Output (part 1): {}", count_black);

    println!("Output (part 2): {}", -1);
}

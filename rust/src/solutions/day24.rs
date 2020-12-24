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

            current = get_coords_in_direction(&dir, &current);
        }

        match grid.get(&current) {
            Some(&'B') => grid.insert(current, 'W'),
            _ => grid.insert(current, 'B'),
        }
    }

    let count_black = grid.iter().filter(|(_p, &x)| x == 'B').count();

    println!("Output (part 1): {}", count_black);

    for _ in 0..100 {
        let mut points_to_flip = Vec::new();

        for x in grid.min_x - 1..=grid.max_x + 1 {
            for y in grid.min_y - 1..=grid.max_y + 1 {
                let current = Point::new(x, y);
                let neighbors_flipped = get_hex_neighbors(&current)
                    .iter()
                    .filter(|&n| grid.get(n).filter(|color| color == &&'B').is_some())
                    .count();

                let should_flip = match grid.get(&current) {
                    Some(&'B') => neighbors_flipped == 0 || neighbors_flipped > 2,
                    _ => neighbors_flipped == 2,
                };

                if should_flip {
                    points_to_flip.push(current);
                }
            }
        }

        for point in points_to_flip.drain(0..) {
            let new_val = match grid.get(&point) {
                Some(&'B') => 'W',
                _ => 'B',
            };
            grid.insert(point, new_val);
        }
    }

    let count_black = grid.iter().filter(|(_p, &x)| x == 'B').count();

    // > 1831
    println!("Output (part 2): {}", count_black);
}

fn get_coords_in_direction(dir: &str, current: &Point) -> Point {
    let east_offset = current.y.abs() % 2;
    let west_offset = 1 - east_offset;

    if dir == "e" {
        Point::new(current.x + 1, current.y)
    } else if dir == "w" {
        Point::new(current.x - 1, current.y)
    } else if dir == "sw" {
        Point::new(current.x - west_offset, current.y + 1)
    } else if dir == "se" {
        Point::new(current.x + east_offset, current.y + 1)
    } else if dir == "nw" {
        Point::new(current.x - west_offset, current.y - 1)
    } else if dir == "ne" {
        Point::new(current.x + east_offset, current.y - 1)
    } else {
        panic!("Why do I like throwing errors?")
    }
}

fn get_hex_neighbors(current: &Point) -> Vec<Point> {
    let east_offset = current.y.abs() % 2;
    let west_offset = 1 - east_offset;

    vec![
        Point::new(current.x + 1, current.y),
        Point::new(current.x - 1, current.y),
        Point::new(current.x - west_offset, current.y + 1),
        Point::new(current.x + east_offset, current.y + 1),
        Point::new(current.x - west_offset, current.y - 1),
        Point::new(current.x + east_offset, current.y - 1),
    ]
}

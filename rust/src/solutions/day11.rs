use adventlib::grid::Point;
use std::iter::repeat;

pub fn solve() {
    println!("Day 11");

    let input_lines = adventlib::read_input_lines("day11input.txt");

    let mut grid: Vec<_> = input_lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut next_grid: Vec<Vec<char>> = Vec::with_capacity(grid.len());
    next_grid.resize_with(grid.len(), || {
        let mut row = Vec::with_capacity(grid[0].len());
        row.resize(grid[0].len(), '.');
        return row;
    });

    let height = grid.len() as i64;
    let width = grid[0].len() as i64;

    loop {
        let mut seat_changed = false;
        for row in 0..height {
            for col in 0..width {
                let adjacent_occupied = Point::new(row, col)
                    .neighbors8()
                    .iter()
                    .map(|point| (point.x, point.y))
                    .filter(|(r, c)| *r >= 0 && *r < height && *c >= 0 && *c < width)
                    .filter(|(r, c)| grid[*r as usize][*c as usize] == '#')
                    .count();
                let current = grid[row as usize][col as usize];
                let next = match (current, adjacent_occupied) {
                    ('L', 0) => '#',
                    ('#', 4..=8) => 'L',
                    _ => current,
                };

                if current != next {
                    seat_changed = true;
                }

                next_grid[row as usize][col as usize] = next;
            }
        }

        if !seat_changed {
            break;
        }

        let t = grid;
        grid = next_grid;
        next_grid = t;
    }

    let mut occupied_count = 0;
    for row in 0..height {
        for col in 0..width {
            if grid[row as usize][col as usize] == '#' {
                occupied_count += 1;
            }
        }
    }
    println!("Output (part 1): {}", occupied_count);

    println!("Output (part 2): {}", -1);
}

use adventlib::grid::Point;

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

    loop {
        let seat_changed = fill_next_grid_with_part_one_rules(&grid, &mut next_grid);
        if !seat_changed {
            break;
        }

        let t = grid;
        grid = next_grid;
        next_grid = t;
    }

    let occupied_count = count_occupied(&grid);
    println!("Output (part 1): {}", occupied_count);

    // reset grid
    let mut grid: Vec<_> = input_lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    loop {
        let seat_changed = fill_next_grid_with_part_two_rules(&grid, &mut next_grid);
        if !seat_changed {
            break;
        }

        let t = grid;
        grid = next_grid;
        next_grid = t;
    }

    let occupied_count = count_occupied(&grid);
    println!("Output (part 2): {}", occupied_count);
}

/* Return true if seat changed */
fn fill_next_grid_with_part_one_rules(
    grid: &Vec<Vec<char>>,
    next_grid: &mut Vec<Vec<char>>,
) -> bool {
    let height = grid.len();
    let width = grid[0].len();

    let mut seat_changed = false;
    for row in 0..height {
        for col in 0..width {
            let adjacent_occupied = Point::new(row as i64, col as i64)
                .neighbors8()
                .iter()
                .filter(|point| is_in_grid_bounds(point, height as i64, width as i64))
                .map(|point| (point.x, point.y))
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

    return seat_changed;
}

fn fill_next_grid_with_part_two_rules(
    grid: &Vec<Vec<char>>,
    next_grid: &mut Vec<Vec<char>>,
) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    let directions = Point::new(0, 0).neighbors8();

    let mut seat_changed = false;
    for row in 0..height {
        for col in 0..width {
            let seat_loc = Point::new(row as i64, col as i64);
            let mut seen_count = 0;
            for direction in directions.iter() {
                let mut viewing_location = seat_loc.clone().vec_add(direction);
                let mut seen_occupied = false;
                while is_in_grid_bounds(&viewing_location, height as i64, width as i64) {
                    let viewed_char =
                        grid[viewing_location.x as usize][viewing_location.y as usize];
                    if viewed_char == '#' {
                        seen_occupied = true;
                        break;
                    }
                    if viewed_char == 'L' {
                        break;
                    }
                    viewing_location = viewing_location.vec_add(direction);
                }

                if seen_occupied {
                    seen_count += 1;
                }
            }

            let current = grid[row as usize][col as usize];
            let next = match (current, seen_count) {
                ('L', 0) => '#',
                ('#', 5..=8) => 'L',
                _ => current,
            };

            if current != next {
                seat_changed = true;
            }

            next_grid[row as usize][col as usize] = next;
        }
    }

    return seat_changed;
}

fn count_occupied(grid: &Vec<Vec<char>>) -> i64 {
    let mut occupied_count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                occupied_count += 1;
            }
        }
    }
    return occupied_count;
}

fn is_in_grid_bounds(point: &Point, first_max: i64, second_max: i64) -> bool {
    point.x >= 0 && point.x < first_max && point.y >= 0 && point.y < second_max
}

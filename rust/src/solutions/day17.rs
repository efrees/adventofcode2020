use adventlib::grid::Point3d;
use std::cmp;
use std::collections::HashMap;

pub fn solve() {
    println!("Day 17");

    let input_lines = adventlib::read_input_lines("day17input_sample.txt");

    let mut grid: HashMap<Point3d, u8> = HashMap::new();
    let mut next_grid: HashMap<Point3d, u8> = HashMap::new();

    let mut min_x = std::i64::MAX;
    let mut min_y = std::i64::MAX;
    let mut min_z = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut max_y = std::i64::MIN;
    let mut max_z = std::i64::MIN;

    // Init
    for j in 0..input_lines.len() {
        let mut i = 0;
        for c in input_lines[j].chars() {
            let point = Point3d::new(i, j as i64, 0);

            min_x = cmp::min(min_x, point.x);
            min_y = cmp::min(min_y, point.y);
            min_z = cmp::min(min_z, point.z);
            max_x = cmp::max(max_x, point.x);
            max_y = cmp::max(max_y, point.y);
            max_z = cmp::max(max_z, point.z);

            grid.insert(
                point,
                match c {
                    '#' => 1,
                    _ => 0,
                },
            );
            i += 1;
        }
    }

    for _cycle in 0..6 {
        println!(
            "checking {} {} {:?}",
            min_z,
            max_z,
            (min_z - 1..=max_z + 1).collect::<Vec<_>>()
        );
        for i in min_x - 1..=max_x + 1 {
            for j in min_y - 1..=max_y + 1 {
                for k in min_z - 1..=max_z + 1 {
                    let point = Point3d::new(i, j, k);

                    let mut count = 0;
                    for n_i in -1..=1 {
                        for n_j in -1..=1 {
                            for n_k in -1..=1 {
                                if n_i == 0 && n_j == 0 && n_k == 0 {
                                    continue;
                                }
                                let neighbor =
                                    Point3d::new(point.x + n_i, point.y + n_j, point.z + n_k);
                                if grid.get(&neighbor) == Some(&1) {
                                    count += 1;
                                }
                            }
                        }
                    }

                    let is_active = grid.get(&point) == Some(&1);
                    let next_is_active = (is_active && count == 2) || count == 3;

                    if next_is_active {
                        next_grid.insert(point, 1);

                        min_x = cmp::min(min_x, point.x);
                        min_y = cmp::min(min_y, point.y);
                        min_z = cmp::min(min_z, point.z);
                        max_x = cmp::max(max_x, point.x);
                        max_y = cmp::max(max_y, point.y);
                        max_z = cmp::max(max_z, point.z);
                    } else {
                        next_grid.insert(point, 0);
                    }
                }
            }
        }

        // println!("{}", render_to_string(&grid, min_x, max_x, min_y, max_y, -1));
        let temp = grid;
        grid = next_grid;
        next_grid = temp;
    }

    let active_count: u32 = grid.values().map(|v| *v as u32).sum();
    println!("Output (part 1): {}", active_count);

    println!("Output (part 2): {}", -1);
}

fn render_to_string(
    grid: &HashMap<Point3d, u8>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    z: i64,
) -> String {
    let number_of_chars = (max_y - min_y) * (max_x - min_x + 1);
    let mut output = String::with_capacity(number_of_chars as usize);

    for i in min_y..=max_y {
        for j in min_x..=max_x {
            let contents = grid.get(&Point3d::new(j, i, z));
            output.push_str(&format!(
                "{}",
                contents
                    .map(|&v| if v == 1 { '#' } else { '.' })
                    .unwrap_or('.')
            ));
        }
        output.push_str("\n");
    }
    return output;
}

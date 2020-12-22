use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct ImageTile {
    id: u32,
    data: Vec<String>,
}

struct TileAlignment {
    tile_id: u32,
    flip: u8,
    rotation: u8,
}

const TILE_SIZE: usize = 10;

pub fn solve() {
    println!("Day 20");

    let input_lines = adventlib::read_input_lines("day20input.txt");

    let mut tiles = Vec::new();

    let mut i = 0;
    while i < input_lines.len() {
        if input_lines[i].starts_with("Tile") {
            tiles.push(parse_tile(&input_lines[i..=i + TILE_SIZE]));
            i += 11;
        }
        i += 1;
    }

    let image_size = num_integer::sqrt(tiles.len());

    let tile_alignments = align_tiles(&tiles, image_size);

    let corner_product = tile_alignments[0].tile_id as u64
        * tile_alignments[image_size - 1].tile_id as u64
        * tile_alignments[image_size * (image_size - 1)].tile_id as u64
        * tile_alignments[image_size * image_size - 1].tile_id as u64;

    println!("Output (part 1): {}", corner_product);

    println!("Output (part 2): {}", -1);
}

fn parse_tile(tile_lines: &[String]) -> ImageTile {
    let header_line = &tile_lines[0];
    let tile_id: u32 = header_line["Tile ".len()..header_line.len() - 1]
        .parse()
        .expect("Must find integer id");
    ImageTile {
        id: tile_id,
        data: tile_lines[1..]
            .iter()
            .map(|line| line.to_string())
            .collect(),
    }
}

fn align_tiles(tiles: &Vec<ImageTile>, image_width: usize) -> Vec<TileAlignment> {
    let mut edge_values: HashMap<u32, Vec<u16>> = HashMap::new();
    for tile in tiles {
        let cw_edge_values = get_edge_values_cw(tile, TILE_SIZE);
        let flipped_edge_values = get_edge_values_ccw(tile, TILE_SIZE);
        let all_edge_values: Vec<u16> = cw_edge_values
            .iter()
            .chain(flipped_edge_values.iter())
            .cloned()
            .collect();
        edge_values.insert(tile.id, all_edge_values);
    }

    let mut alignments = Vec::new();

    if !align_tiles_rec(tiles, image_width, &mut alignments, &edge_values) {
        panic!("No solution found.");
    }

    return alignments;
}

fn align_tiles_rec(
    tiles: &Vec<ImageTile>,
    image_width: usize,
    alignments: &mut Vec<TileAlignment>,
    edge_values: &HashMap<u32, Vec<u16>>,
) -> bool {
    if alignments.len() == tiles.len() {
        return true;
    }

    let next_index = alignments.len();
    let used_tiles: HashSet<u32> = alignments.iter().map(|tile| tile.tile_id).collect();

    for tile in tiles {
        if used_tiles.contains(&tile.id) {
            continue;
        }

        let tile_edges = edge_values.get(&tile.id).unwrap();

        if next_index >= image_width {
            let tile_above = &alignments[next_index - image_width];
            let top_edge = get_bottom_edge(tile_above, edge_values);

            for edge_index in 0..8u8 {
                if tile_edges[edge_index as usize] != top_edge {
                    continue;
                }

                // actually matches corresponding flipped value since top is LTR and bottom is RTL
                let flipped_index = 8 - 1 - edge_index;
                let potential_alignment = TileAlignment {
                    tile_id: tile.id,
                    flip: flipped_index / 4,
                    rotation: flipped_index % 4,
                };

                // Ensure left side matches
                if (next_index % image_width) > 0 {
                    let tile_at_left = &alignments[next_index - 1];
                    let right_edge = get_right_edge(tile_at_left, edge_values);

                    // Right edge is read top to bottom, so we compare with the inverse of our left edge
                    if right_edge != get_left_edge_inverse(&potential_alignment, edge_values) {
                        continue;
                    }
                }

                alignments.push(potential_alignment);
                if align_tiles_rec(tiles, image_width, alignments, edge_values) {
                    return true;
                }
                alignments.pop();
            }
        } else if next_index > 0 {
            let tile_at_left = &alignments[next_index - 1];
            let left_edge = get_right_edge(tile_at_left, edge_values);

            for edge_index in 0..8u8 {
                if tile_edges[edge_index as usize] != left_edge {
                    continue;
                }

                // actually matches corresponding flipped value since right is top-to-bottom and left is bottom-to-top
                let flipped_index = 8 - 1 - edge_index;
                let potential_alignment = TileAlignment {
                    tile_id: tile.id,
                    flip: flipped_index / 4,
                    rotation: (flipped_index + 4 - 3) % 4,
                };

                alignments.push(potential_alignment);
                if align_tiles_rec(tiles, image_width, alignments, edge_values) {
                    return true;
                }
                alignments.pop();
            }
        } else {
            // First one. Try all rotations
            // (but no need to flip since we can build the image from the "front" or "back");
            for edge_index in 0..4u8 {
                let potential_alignment = TileAlignment {
                    tile_id: tile.id,
                    flip: 0,
                    rotation: edge_index,
                };

                alignments.push(potential_alignment);
                if align_tiles_rec(tiles, image_width, alignments, edge_values) {
                    return true;
                }
                alignments.pop();
            }
        }
    }

    false
}

/// returns [top, right, bottom, left]
fn get_edge_values_cw(tile: &ImageTile, tile_size: usize) -> [u16; 4] {
    let mut top = 0;
    let mut right = 0;
    let mut bottom = 0;
    let mut left = 0;

    for i in 0..tile_size {
        top = top * 2 + pixel_value(tile.data[0].chars().nth(i));
        right = right * 2 + pixel_value(tile.data[i].chars().nth(tile_size - 1));
        bottom = bottom * 2 + pixel_value(tile.data[tile_size - 1].chars().nth(tile_size - i - 1));
        left = left * 2 + pixel_value(tile.data[tile_size - i - 1].chars().next());
    }

    [top, right, bottom, left]
}

/// returns [left', bottom', right', top']
fn get_edge_values_ccw(tile: &ImageTile, tile_size: usize) -> [u16; 4] {
    // swap axes by mirroring and rotating once CCW
    let flipped_tile_data: Vec<String> = tile
        .data
        .iter()
        .map(|row| row.chars().rev().collect())
        .collect();
    let flipped_tile = ImageTile {
        id: tile.id,
        data: flipped_tile_data,
    };

    let mut values = get_edge_values_cw(&flipped_tile, tile_size);
    values.rotate_left(1);
    values
}

fn get_bottom_edge(alignment: &TileAlignment, edge_values: &HashMap<u32, Vec<u16>>) -> u16 {
    let edges = edge_values.get(&alignment.tile_id).unwrap();
    let edge_index = alignment.flip * 4 + (alignment.rotation + 2) % 4;
    edges[edge_index as usize]
}

fn get_right_edge(alignment: &TileAlignment, edge_values: &HashMap<u32, Vec<u16>>) -> u16 {
    let edges = edge_values.get(&alignment.tile_id).unwrap();
    let edge_index = alignment.flip * 4 + (alignment.rotation + 1) % 4;
    edges[edge_index as usize]
}

fn get_left_edge_inverse(alignment: &TileAlignment, edge_values: &HashMap<u32, Vec<u16>>) -> u16 {
    // one flip will put the left, flipped, on the top
    let edges = edge_values.get(&alignment.tile_id).unwrap();
    let edge_index = (1 - alignment.flip) * 4 + (4 - alignment.rotation) % 4;
    edges[edge_index as usize]
}

fn pixel_value(pixel_raw: Option<char>) -> u16 {
    match pixel_raw {
        Some('#') => 1,
        _ => 0,
    }
}

/*
Want left (3)
0 rotation -> 1 flip, get 0
1 rotation -> 1 flip, get 3
2 rotations -> 1 flip, get 2
3 rotations -> 1 flip, get 1

1 flip, 0 -> 0 flip, 0
1 flip, 1 -> 0 flip, 3
1 flip, 2 -> 0 flip, 2
1 flip, 3 -> 0 flip, 1
*/

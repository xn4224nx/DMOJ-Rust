/*
 * CCC '03 S3 - Floor Plan
 * https://dmoj.ca/problem/ccc03s3
 */

use std::collections::HashSet;

const WALL: char = 'I';
const FLOOR: char = '.';

fn main() {
    let mut buffer = String::new();

    /* Read the area of flooring available and the dimensions of the plan. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let in_data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut floor_plan: Vec<Vec<char>> = Vec::new();

    /* Read the floor plan */
    for _ in 0..in_data[1] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        floor_plan.push(buffer.trim_end().chars().collect());
    }

    /* Determine the number of rooms and their size. */
    let mut used_coords = HashSet::new();
    let mut room_sizes = Vec::new();

    for row in 0..in_data[2] {
        for col in 0..in_data[1] {
            if floor_plan[col][row] == WALL || used_coords.contains(&(col, row)) {
                continue;
            }
            used_coords.insert((col, row));

            /* Search for other connected points in the room. */
            let mut curr_pnts = vec![(col, row)];
            let mut room_size = 1;

            while !curr_pnts.is_empty() {
                let mut nxt_pnts = Vec::new();

                /* Check the points up, down, left and right to these ones. */
                for (c_col, c_row) in curr_pnts.drain(..) {
                    /* The point right above. */
                    if c_col > 0 && !used_coords.contains(&(c_col - 1, c_row)) {
                        if floor_plan[c_col - 1][c_row] == FLOOR {
                            nxt_pnts.push((c_col - 1, c_row));
                            room_size += 1;
                        }
                        used_coords.insert((c_col - 1, c_row));
                    }
                    /* The point right below. */
                    if c_col < in_data[1] - 1 && !used_coords.contains(&(c_col + 1, c_row)) {
                        if floor_plan[c_col + 1][c_row] == FLOOR {
                            nxt_pnts.push((c_col + 1, c_row));
                            room_size += 1;
                        }
                        used_coords.insert((c_col + 1, c_row));
                    }
                    /* The point to the left. */
                    if c_row > 0 && !used_coords.contains(&(c_col, c_row - 1)) {
                        if floor_plan[c_col][c_row - 1] == FLOOR {
                            nxt_pnts.push((c_col, c_row - 1));
                            room_size += 1;
                        }
                        used_coords.insert((c_col, c_row - 1));
                    }
                    /* The point to the right. */
                    if c_row < in_data[2] - 1 && !used_coords.contains(&(c_col, c_row + 1)) {
                        if floor_plan[c_col][c_row + 1] == FLOOR {
                            nxt_pnts.push((c_col, c_row + 1));
                            room_size += 1;
                        }
                        used_coords.insert((c_col, c_row + 1));
                    }
                }
                curr_pnts = nxt_pnts;
            }
            room_sizes.push(room_size);
        }
    }

    /* Sort the rooms largest to smallest. */
    room_sizes.sort();
    room_sizes.reverse();

    /* Determine how many rooms are completly covered. */
    let mut floor_left = in_data[0];
    let mut rooms_covered = 0;

    for rm_sz in room_sizes {
        if rm_sz > floor_left {
            break;
        }
        rooms_covered += 1;
        floor_left -= rm_sz;
    }

    /* Show the results. */
    if rooms_covered == 1 {
        println!(
            "{} room, {} square metre(s) left over",
            rooms_covered, floor_left
        );
    } else {
        println!(
            "{} rooms, {} square metre(s) left over",
            rooms_covered, floor_left
        );
    }
}

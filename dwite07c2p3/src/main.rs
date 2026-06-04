/*
 * DWITE '07 R2 #3 - Floor Plan
 * https://dmoj.ca/problem/dwite07c2p3
 */

use std::collections::HashSet;

struct Floor {
    free_space: char,
    room_idx_locs: Vec<(usize, usize)>,
    contents: Vec<Vec<char>>,
}

impl Floor {
    fn new(free_space: char) -> Self {
        let mut rooms = Vec::new();
        let mut contents = Vec::new();
        let mut buffer = String::new();

        /* Read the floor dimensions from STDIN. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let dims = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Read the floor contents from STDIN. */
        for row_idx in 0..dims[1] {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();
            let mut line = Vec::new();

            /* Remove the newline character. */
            buffer.pop();

            /* Check this line for room indexes. */
            for (col_idx, sqr) in buffer.chars().enumerate() {
                if let Some(val) = sqr.to_digit(10) {
                    rooms.push(((row_idx, col_idx), val));
                }
                line.push(sqr);
            }
            contents.push(line);
        }

        /* The rooms should be referenced smallest to largest by index. */
        rooms.sort_by_key(|&(_, idx)| idx);

        return Floor {
            free_space,
            room_idx_locs: rooms.into_iter().map(|x| x.0).collect(),
            contents,
        };
    }

    /// For each room on the floor count the tiles and return a vector of the sizes
    /// with the index coresponding to the room with that number.
    fn calc_room_sizes(&self) -> Vec<usize> {
        let mut total_room_sizes = Vec::new();

        /* Count the free spaces for each room in turn. */
        for room_idx in 0..self.room_idx_locs.len() {
            let mut visited_sqrs = HashSet::new();
            let mut cur_sqrs = vec![self.room_idx_locs[room_idx]];

            /* Find all spaces attached to the index square. */
            while !cur_sqrs.is_empty() {
                let mut new_sqrs = Vec::new();

                for (s_row, s_col) in cur_sqrs.drain(..) {
                    visited_sqrs.insert((s_row, s_col));

                    /* Look above. */
                    if s_row > 0
                        && self.contents[s_row - 1].len() > s_col
                        && self.contents[s_row - 1][s_col] == self.free_space
                        && !visited_sqrs.contains(&(s_row - 1, s_col))
                    {
                        new_sqrs.push((s_row - 1, s_col));
                    }

                    /* look below. */
                    if s_row < self.contents.len() - 1
                        && self.contents[s_row + 1].len() > s_col
                        && self.contents[s_row + 1][s_col] == self.free_space
                        && !visited_sqrs.contains(&(s_row + 1, s_col))
                    {
                        new_sqrs.push((s_row + 1, s_col));
                    }

                    /* Look to the left. */
                    if s_col > 0
                        && self.contents[s_row][s_col - 1] == self.free_space
                        && !visited_sqrs.contains(&(s_row, s_col - 1))
                    {
                        new_sqrs.push((s_row, s_col - 1));
                    }

                    /* look to the right */
                    if s_col < self.contents[s_row].len() - 1
                        && self.contents[s_row][s_col + 1] == self.free_space
                        && !visited_sqrs.contains(&(s_row, s_col + 1))
                    {
                        new_sqrs.push((s_row, s_col + 1));
                    }
                }

                /* Prepare for the next iteration of the loop. */
                cur_sqrs = new_sqrs;
            }
            total_room_sizes.push(visited_sqrs.len());
        }
        return total_room_sizes;
    }
}

fn main() {
    println!(
        "{}",
        Floor::new('.')
            .calc_room_sizes()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

/*
 * CCC '20 S2 - Escape Room
 * https://dmoj.ca/problem/ccc20s2
 */

use std::collections::{HashMap, HashSet};

fn main() {
    let mut buffer = String::new();

    /* What is the size of the grid? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let grid_dims = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut grid_val_lookup: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut grid = vec![vec![0; grid_dims[1]]; grid_dims[0]];

    /* Read the grid of numbers. */
    for row_idx in 0..grid_dims[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        for (col_idx, value) in buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .into_iter()
            .enumerate()
        {
            grid_val_lookup
                .entry(value)
                .and_modify(|x| x.push((row_idx, col_idx)))
                .or_insert(vec![(row_idx, col_idx)]);

            grid[row_idx][col_idx] = (row_idx + 1) * (col_idx + 1);
        }
    }

    /* Keep track of the cells that have been visited. */
    let mut visited = vec![vec![false; grid_dims[1]]; grid_dims[0]];

    /* Start at the destination and try and reach the beginning. */
    let mut curr_points = HashSet::from([(grid_dims[0] - 1, grid_dims[1] - 1)]);

    /* Attempt to traverse the room from one corner to the other. */
    while !curr_points.is_empty() {
        let mut nxt_points = HashSet::new();

        /* Determine where the next jump takes you. */
        for c_pnt in curr_points.drain() {
            if visited[c_pnt.0][c_pnt.1] {
                continue;
            } else {
                visited[c_pnt.0][c_pnt.1] = true;
            };

            /* If the result isn't in the grid skip it. */
            let Some(next_points) = grid_val_lookup.get(&grid[c_pnt.0][c_pnt.1]) else {
                continue;
            };

            /* Get the next possible jumps from this point. */
            for n_pnt in next_points.into_iter() {
                if !visited[n_pnt.0][n_pnt.1] {
                    nxt_points.insert(*n_pnt);

                    /* Has the end point been reached? */
                    if n_pnt.0 == 0 && n_pnt.1 == 0 {
                        println!("yes");
                        return;
                    }
                }
            }
        }
        curr_points = nxt_points;
    }

    /* All pathways have been exhausted and the end has not been reached. */
    println!("no");
}

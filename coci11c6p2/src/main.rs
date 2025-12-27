/*
 * COCI '11 Contest 6 #2 Prozori
 * https://dmoj.ca/problem/coci11c6p2
 */

const WINDOW_H: usize = 4;
const WINDOW_W: usize = 4;
const BLIND_CHR: char = '*';

fn main() {
    let mut buffer = String::new();

    /* Read the dimensions. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let dims = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Keep a record of what state each window is in. */
    let mut windows = vec![vec![0; dims[1]]; dims[0]];

    /* Read the windows */
    for line_idx in 0..(dims[0] * WINDOW_H + dims[0] + 1) {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let row_data = buffer.chars().collect::<Vec<char>>();

        /* Skip wall rows */
        if line_idx % (WINDOW_H + 1) == 0 {
            continue;
        }

        /* Determine what row of windows this line corresponds to. */
        let row = line_idx / (WINDOW_H + 1);

        /* Determine what state the windows are in on this line.  */
        for wind_idx in 0..dims[1] {
            let col_idx = wind_idx * (WINDOW_W + 1) + 1;
            if row_data[col_idx] == BLIND_CHR {
                windows[row][wind_idx] += 1;
            }
        }
    }

    /* Count the number of each state. */
    let mut window_state = vec![0; WINDOW_H + 1];
    for state in windows.into_iter().flatten() {
        window_state[state] += 1;
    }

    /* Display the state of all windows. */
    println!(
        "{}",
        window_state
            .into_iter()
            .map(|x: usize| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

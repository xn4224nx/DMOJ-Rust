/*
 * COCI '12 Contest 3 #3 Malcolm
 * https://dmoj.ca/problem/coci12c3p3
 */

use std::collections::VecDeque;

fn main() {
    let mut pairs = 0;
    let mut buffer = String::new();

    /* Read the number of students and the friendship range. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut name_lens: Vec<VecDeque<usize>> = vec![VecDeque::new(); 20];

    /* Read the student names and check if they are friends. */
    for f_idx in 0..data[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let n_len = buffer.len()-2;

        /* Remove  names that are not relevant. */
        while !name_lens[n_len].is_empty() && f_idx - name_lens[n_len][0] > data[1] {
            name_lens[n_len].pop_front().unwrap();
        }

        /* Count the number of good friends with the people above. */
        pairs += name_lens[n_len].len();

        /* Save this name for future comparison. */
        name_lens[n_len].push_back(f_idx);
    }
    println!("{}", pairs);
}

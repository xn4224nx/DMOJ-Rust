/*
 * https://dmoj.ca/problem/ccc18j2
 */

use std::io;

fn main() {
    let mut buffer = String::new();

    /* Read the number of lines to be compressed. */
    io::stdin().read_line(&mut buffer).unwrap();
    let num_lines = buffer.trim().parse::<usize>().unwrap();

    /* Read the lines to be compressed. */
    for _ in 0..num_lines {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let un_comp: Vec<char> = buffer.trim().chars().collect();
        let mut curr_char = un_comp[0];
        let mut curr_cnt = 1;

        /* Iterate over the line char by char. */
        for ch_idx in 1..un_comp.len() {
            if un_comp[ch_idx] == curr_char {
                curr_cnt += 1;
            } else {
                print!("{} {} ", curr_cnt, curr_char);
                curr_cnt = 1;
                curr_char = un_comp[ch_idx];
            }
        }
        println!("{} {}", curr_cnt, curr_char);
    }
}

/*
 * https://dmoj.ca/problem/ccc18j2
 */

use std::io;

fn main() {
    let mut input = String::new();
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut same_space_cnt = 0;

    /* Read the input. */
    for _ in 0..3 {
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        data.push(input.chars().collect());
        input.clear();
    }

    /* Find number of parking spaces which were occupied yesterday and today */
    for sp_idx in 0..data[1].len() {
        if data[1][sp_idx] == 'C' && data[2][sp_idx] == 'C' {
            same_space_cnt += 1;
        }
    }
    println!("{}", same_space_cnt);
}

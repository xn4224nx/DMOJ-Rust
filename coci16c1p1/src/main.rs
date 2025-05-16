/*
 * COCI '16 Contest 1 #1 Tarifa
 *
 * https://dmoj.ca/problem/coci16c1p1
 */

use std::io;

fn main() {
    let mut input = String::new();

    /* Read the monthly allowance. */
    io::stdin().read_line(&mut input).unwrap();
    input.pop();
    let allowance = input.parse::<u32>().unwrap();
    input.clear();

    /* How many months data are we going to recieve. */
    io::stdin().read_line(&mut input).unwrap();
    input.pop();
    let months = input.parse::<usize>().unwrap();
    input.clear();

    /* Calculate the final months allowance */
    let mut running_overlap = 0;
    for _ in 0..months {
        running_overlap += allowance;
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        running_overlap -= input.parse::<u32>().unwrap();
        input.clear();
    }
    println!("{}", running_overlap + allowance);
}

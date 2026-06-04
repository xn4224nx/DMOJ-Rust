/*
 * COCI '25 Contest 5 #1 Škare
 * https://dmoj.ca/problem/coci25c5p1
 */

use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();

    /* Read the length of the strip and the number of instructions. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    assert_eq!(data.len(), 2);

    let mut all_strips = vec![data[0]];

    /* Read the cuts. */
    for _ in 0..data[1] {
        buffer.clear();

        /* Read the length of the strip at that moment and where the cut will be. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let strip_data = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Add in the new strip after the original */
        all_strips.insert(strip_data[0], all_strips[strip_data[0] - 1] - strip_data[1]);

        /* Cut the large strip down. */
        all_strips[strip_data[0] - 1] = strip_data[1];
    }

    /* Determine the number of unique strip lengths. */
    println!("{}", all_strips.drain(..).collect::<HashSet<usize>>().len());
}

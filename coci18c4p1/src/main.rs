/*
 * COCI '18 Contest 4 #1 Elder
 * https://dmoj.ca/problem/coci18c4p1
 */

use std::collections::HashSet;

fn main() {
    let mut input = String::new();

    /* What wizard originaly had the elder wand? */
    std::io::stdin().read_line(&mut input).unwrap();
    let mut eldar_owner = input.chars().next().unwrap();
    let mut all_owners = HashSet::from([eldar_owner]);
    input.clear();

    /* How many lines of data are there? */
    std::io::stdin().read_line(&mut input).unwrap();
    let num_lines = input.trim().parse::<usize>().unwrap();
    input.clear();

    /* Simulate the wizard duels. */
    for _ in 0..num_lines {
        std::io::stdin().read_line(&mut input).unwrap();
        let line_vec: Vec<char> = input.chars().collect();

        /* The eldar wand swaps wizards. */
        if eldar_owner == line_vec[2] {
            eldar_owner = line_vec[0];
            all_owners.insert(eldar_owner);
        }
        input.clear();
    }
    println!("{}\n{}", eldar_owner, all_owners.len())
}

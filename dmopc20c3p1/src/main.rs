/*
 * DMOPC '20 Contest 3 P1 - Present Checking
 * https://dmoj.ca/problem/dmopc20c3p1
 *
 */

use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();

    /* Read the total number of friends. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_friends = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the numbered gifts. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let gifts = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();

    /* If there are duplicates then not everyone has a gift. */
    println!(
        "{}",
        if gifts.len() == num_friends {
            "YES"
        } else {
            "NO"
        }
    );
}

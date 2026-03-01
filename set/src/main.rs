/*
 * Unique Elements
 * https://dmoj.ca/problem/set
 */

use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();

    /* How many elements will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_ele = buffer.trim_end().parse::<usize>().unwrap();

    let mut uniq_ele = HashSet::with_capacity(num_ele);

    /* Read and deduplicate the elements. */
    for _ in 0..num_ele {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        uniq_ele.insert(buffer.trim_end().parse::<usize>().unwrap());
    }
    println!("{}", uniq_ele.len());
}

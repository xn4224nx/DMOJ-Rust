/*
 * COCI '14 Contest 2 #2 Utrka
 * https://dmoj.ca/problem/coci14c2p2
 */

use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();

    /* Read the number of runners. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_runners: usize = buffer.trim().parse::<usize>().unwrap();

    /* Keep a record of all the runners. */
    let mut all_runners: HashMap<String, usize> = HashMap::with_capacity(num_runners);

    /* Read the names of the runners. */
    for _ in 0..num_runners {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        all_runners
            .entry(buffer.clone())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    /* Remove names of runners that completed the race. */
    for _ in 0..num_runners - 1 {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        *all_runners.get_mut(&buffer).unwrap() -= 1;
    }

    /* Who was the runner who did not complete the race */
    for (runner, count) in all_runners.drain() {
        if count == 1 {
            print!("{}", runner);
            break;
        }
    }
}

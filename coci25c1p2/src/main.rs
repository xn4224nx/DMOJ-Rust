/*
 * COCI '25 Contest 1 #2 Krugomet
 * https://dmoj.ca/problem/coci25c1p2
 */

use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();
    let mut seen_states = HashMap::new();

    /* Read the number of rounds and the number of players. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let metadata = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Read the inital starting pool amounts. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut pools = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Read the sinks for each pool. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let sinks = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect::<Vec<usize>>();

    assert_eq!(pools.len(), sinks.len());
    let mut cyc_idx = 0;
    let mut loop_found = false;

    /* Simulate the draining of the pools for a set number of cycles. */
    while cyc_idx < metadata[1] {
        /* Has this state been seem before? */
        if seen_states.contains_key(&pools) {
            loop_found = true;
            break;
        } else {
            seen_states.insert(pools.clone(), cyc_idx);
        }

        /* Each pool is completly drained to its sink. */
        perform_cycle(&mut pools, &sinks);

        /* Prepare for the next loop iteration. */
        cyc_idx += 1;
    }

    /* If a loop has been found, determine the final pool. */
    if loop_found {
        let loop_len = cyc_idx - seen_states.get(&pools).unwrap();
        let remaining_cyc = (metadata[1] - cyc_idx) % loop_len;
        for _ in 0..remaining_cyc {
            perform_cycle(&mut pools, &sinks)
        }
    }

    /* What is the most full pool. */
    let pool_max = *pools.iter().max().unwrap();
    println!("{}", pool_max);

    /* Find the indexes of the most full pool. */
    for pl_idx in 0..pools.len() {
        if pools[pl_idx] == pool_max {
            println!("{}", pl_idx + 1);
        }
    }
}

/// Simulate the pools being drained once.
fn perform_cycle(pool: &mut Vec<usize>, sinks: &Vec<usize>) {
    let mut new_pool = vec![0; pool.len()];

    /* Each pool is completly drained to its sink. */
    for (pl_idx, pl_val) in pool.drain(..).enumerate() {
        new_pool[sinks[pl_idx]] += pl_val
    }

    /* Overwrite the original with the new state. */
    *pool = new_pool;
}

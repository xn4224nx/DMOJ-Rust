/*
 * The Mirror
 * https://dmoj.ca/problem/phantom1
 */

use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();

    /* How many pairs will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_pairs = buffer.trim_end().parse::<usize>().unwrap();

    let mut limit_pairs: Vec<(usize, usize)> = Vec::with_capacity(num_pairs);
    let mut max_limt = 0;

    /* Read the pairs of numbers. */
    for _ in 0..num_pairs {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let raw_pair = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        limit_pairs.push((raw_pair[0], raw_pair[1]));

        /* Record the maximum value */
        if raw_pair[1] > max_limt {
            max_limt = raw_pair[1];
        }
    }

    /* Determine the primes. */
    let primes = sieve_of_eratosthenes(max_limt);

    /* Print the number of primes between each pair. */
    for n_pair in limit_pairs {
        println!(
            "{}",
            (n_pair.0..n_pair.1).filter(|x| primes.contains(&x)).count()
        );
    }
}

/// Find the primes from two all the way to max value and return them in a
/// HashSet.
pub fn sieve_of_eratosthenes(max_value: usize) -> HashSet<usize> {
    let mut is_prime = vec![true; max_value + 1];
    let mut num = 2;

    /* Rule out multiples up to half of the max. */
    while num < max_value / 2 {
        let mut chec_idx = 2 * num;

        /* Rule out numbers that are multiples of this number. */
        while chec_idx < max_value + 1 {
            is_prime[chec_idx] = false;
            chec_idx += num;
        }
        num += 1;
    }

    /* Convert the array to a HashSet. */
    return is_prime
        .into_iter()
        .enumerate()
        .filter(|(x, y)| *y && *x > 1)
        .map(|(x, _)| x)
        .collect();
}

/*
 * New Year's '18 P2 - Mimi and Christmas Cake
 * https://dmoj.ca/problem/year2018p2
 */

use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();

    /* Read the line of numbers. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let nums_to_check = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();

    /* Determine the largest value in nums. */
    let max_num = nums_to_check.iter().max().unwrap();

    /* Get the prime numbers that could be in the numbers. */
    let primes = sieve_of_eratosthenes(*max_num);

    println!(
        "{}",
        primes
            .intersection(&nums_to_check)
            .collect::<HashSet<&usize>>()
            .len()
    );
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

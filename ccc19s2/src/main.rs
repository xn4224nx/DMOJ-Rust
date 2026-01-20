/*
 * CCC '19 S2 - Pretty Average Primes
 * https://dmoj.ca/problem/ccc19s2
 */

use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();

    /* How many test cases are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_cases = buffer.trim_end().parse::<usize>().unwrap();
    buffer.clear();

    /* Read the test cases. */
    for _ in 0..num_cases {
        std::io::stdin().read_line(&mut buffer).unwrap();
    }

    /* Parse the test cases. */
    let case_vals = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Find all the prime numbers that will be needed */
    let eth_primes = sieve_of_eratosthenes(2 * case_vals.iter().max().unwrap());

    /* Determine two prime numbers that sum to the case value. */
    for val in case_vals.into_iter() {
        let av_primes = find_prime_average(val, &eth_primes);
        println!("{} {}", av_primes.0, av_primes.1);
    }
}

/// Get all the prime numbers under a certain limit.
pub fn sieve_of_eratosthenes(limit: usize) -> HashSet<usize> {
    let mut primes = vec![true; limit];
    let mut multi = 2;

    /* Zero and one are not prime numbers. */
    primes[0] = false;
    primes[1] = false;

    while multi * multi < limit {
        let mut test_value = multi * 2;

        /* All multiples except the first one are not primes. */
        while test_value < limit {
            primes[test_value] = false;
            test_value += multi;
        }
        multi += 1;
    }
    return primes
        .into_iter()
        .enumerate()
        .filter(|(_, is_prime)| *is_prime)
        .map(|(num, _)| num)
        .collect();
}

/// Find two prime numbers who's mean value is a specific number
pub fn find_prime_average(avg_val: usize, primes: &HashSet<usize>) -> (usize, usize) {
    for prime_0 in primes.iter() {
        if 2 * avg_val > *prime_0 {
            let pos_prime = 2 * avg_val - *prime_0;

            /* Test the other side of the average to see if its a prime. */
            if pos_prime == *prime_0 || primes.contains(&pos_prime) {
                return (*prime_0, pos_prime);
            }
        }
    }
    panic!("Goldbach's Conjecture is False! /s");
}

/*
 * Tests for main.rs
 */

#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod ccc19s2;
use assert_cmd::{Command, pkg_name};
use ccc19s2::{find_prime_average, sieve_of_eratosthenes};
use std::collections::HashSet;

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("4\n", "8\n", "4\n", "7\n", "21\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("3 13\n", "3 5\n", "7 7\n", "13 29\n",));
}

#[test]
fn sieve_of_eratosthenes_exp0() {
    assert_eq!(
        sieve_of_eratosthenes(30),
        HashSet::from([2, 3, 5, 7, 11, 13, 17, 19, 23, 29,])
    );
}

#[test]
fn sieve_of_eratosthenes_exp1() {
    assert_eq!(sieve_of_eratosthenes(10), HashSet::from([2, 3, 5, 7]));
}

#[test]
fn sieve_of_eratosthenes_exp2() {
    assert_eq!(
        sieve_of_eratosthenes(100),
        HashSet::from([
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97
        ])
    );
}

#[test]
fn sieve_of_eratosthenes_exp3() {
    assert_eq!(
        sieve_of_eratosthenes(660),
        HashSet::from([
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659
        ])
    );
}

#[test]
fn sieve_of_eratosthenes_exp4() {
    assert_eq!(
        sieve_of_eratosthenes(28),
        HashSet::from([2, 3, 5, 7, 11, 13, 17, 19, 23])
    );
}

#[test]
fn find_prime_average_exp0() {
    assert_eq!(
        find_prime_average(
            8,
            &HashSet::from([
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ])
        ),
        (3, 13)
    );
}

#[test]
fn find_prime_average_exp1() {
    assert_eq!(
        find_prime_average(
            4,
            &HashSet::from([
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ])
        ),
        (3, 5)
    );
}

#[test]
fn find_prime_average_exp2() {
    assert_eq!(
        find_prime_average(
            7,
            &HashSet::from([
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ])
        ),
        (7, 7)
    );
}

#[test]
fn find_prime_average_exp3() {
    assert_eq!(
        find_prime_average(
            21,
            &HashSet::from([
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ])
        ),
        (13, 29)
    );
}

/*
 * Tests for main.rs
 */

#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod ccc10j5;
use assert_cmd::{Command, pkg_name};
use ccc10j5::{find_min_jumps, next_moves};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("2 1\n", "3 3\n",)).assert();
    assert.success().stdout(concat!("1\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("4 2\n", "7 5\n",)).assert();
    assert.success().stdout(concat!("2\n",));
}

#[test]
fn next_moves_exp0() {
    assert_eq!(
        next_moves(&(3, 3)),
        vec![
            (4, 5),
            (5, 4),
            (5, 2),
            (4, 1),
            (2, 1),
            (1, 2),
            (1, 4),
            (2, 5),
        ]
    );
}

#[test]
fn next_moves_exp1() {
    assert_eq!(next_moves(&(0, 0)), vec![(1, 2), (2, 1),]);
}

#[test]
fn next_moves_exp2() {
    assert_eq!(next_moves(&(7, 7)), vec![(6, 5), (5, 6),]);
}

#[test]
fn find_min_jumps_exp0() {
    assert_eq!(find_min_jumps(&(3, 3), &(4, 5)), 1);
}

#[test]
fn find_min_jumps_exp1() {
    assert_eq!(find_min_jumps(&(1, 0), &(2, 2)), 1);
}

#[test]
fn find_min_jumps_exp2() {
    assert_eq!(find_min_jumps(&(3, 1), &(6, 4)), 2);
}

#[test]
fn find_min_jumps_exp3() {
    assert_eq!(find_min_jumps(&(4, 5), &(3, 3)), 1);
}

#[test]
fn find_min_jumps_exp4() {
    assert_eq!(find_min_jumps(&(2, 2), &(1, 0)), 1);
}

#[test]
fn find_min_jumps_exp5() {
    assert_eq!(find_min_jumps(&(6, 4), &(3, 1)), 2);
}

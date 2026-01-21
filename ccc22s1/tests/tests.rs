/*
 * Tests for main.rs
 */

#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod ccc19s2;
use assert_cmd::{Command, pkg_name};
use ccc19s2::sum_comb;

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("14\n",)).assert();
    assert.success().stdout(concat!("1\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("40\n",)).assert();
    assert.success().stdout(concat!("3\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("6\n",)).assert();
    assert.success().stdout(concat!("0\n",));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("20\n",)).assert();
    assert.success().stdout(concat!("2\n",));
}

#[test]
fn sum_comb_exp00() {
    assert_eq!(sum_comb(4, 5, 1), 0);
}

#[test]
fn sum_comb_exp01() {
    assert_eq!(sum_comb(4, 5, 2), 0);
}

#[test]
fn sum_comb_exp02() {
    assert_eq!(sum_comb(4, 5, 3), 0);
}

#[test]
fn sum_comb_exp03() {
    assert_eq!(sum_comb(4, 5, 4), 1);
}

#[test]
fn sum_comb_exp04() {
    assert_eq!(sum_comb(4, 5, 5), 1);
}

#[test]
fn sum_comb_exp05() {
    assert_eq!(sum_comb(4, 5, 6), 0);
}

#[test]
fn sum_comb_exp06() {
    assert_eq!(sum_comb(4, 5, 7), 0);
}

#[test]
fn sum_comb_exp07() {
    assert_eq!(sum_comb(4, 5, 8), 1);
}

#[test]
fn sum_comb_exp08() {
    assert_eq!(sum_comb(4, 5, 9), 1);
}

#[test]
fn sum_comb_exp09() {
    assert_eq!(sum_comb(4, 5, 10), 1);
}

#[test]
fn sum_comb_exp10() {
    assert_eq!(sum_comb(4, 5, 14), 1);
}

#[test]
fn sum_comb_exp11() {
    assert_eq!(sum_comb(4, 5, 20), 2);
}

#[test]
fn sum_comb_exp12() {
    assert_eq!(sum_comb(4, 5, 40), 3);
}

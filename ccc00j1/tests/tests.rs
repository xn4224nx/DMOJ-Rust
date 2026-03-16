/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("3 30\n",)).assert();
    assert.success().stdout(concat!(
        "Sun Mon Tue Wed Thr Fri Sat\n",
        "          1   2   3   4   5\n",
        "  6   7   8   9  10  11  12\n",
        " 13  14  15  16  17  18  19\n",
        " 20  21  22  23  24  25  26\n",
        " 27  28  29  30\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1 28\n",)).assert();
    assert.success().stdout(concat!(
        "Sun Mon Tue Wed Thr Fri Sat\n",
        "  1   2   3   4   5   6   7\n",
        "  8   9  10  11  12  13  14\n",
        " 15  16  17  18  19  20  21\n",
        " 22  23  24  25  26  27  28\n",
    ));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1 29\n",)).assert();
    assert.success().stdout(concat!(
        "Sun Mon Tue Wed Thr Fri Sat\n",
        "  1   2   3   4   5   6   7\n",
        "  8   9  10  11  12  13  14\n",
        " 15  16  17  18  19  20  21\n",
        " 22  23  24  25  26  27  28\n",
        " 29\n",
    ));
}

#[test]
fn full_program_exp3() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1 30\n",)).assert();
    assert.success().stdout(concat!(
        "Sun Mon Tue Wed Thr Fri Sat\n",
        "  1   2   3   4   5   6   7\n",
        "  8   9  10  11  12  13  14\n",
        " 15  16  17  18  19  20  21\n",
        " 22  23  24  25  26  27  28\n",
        " 29  30\n",
    ));
}

#[test]
fn full_program_exp4() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1 31\n",)).assert();
    assert.success().stdout(concat!(
        "Sun Mon Tue Wed Thr Fri Sat\n",
        "  1   2   3   4   5   6   7\n",
        "  8   9  10  11  12  13  14\n",
        " 15  16  17  18  19  20  21\n",
        " 22  23  24  25  26  27  28\n",
        " 29  30  31\n",
    ));
}

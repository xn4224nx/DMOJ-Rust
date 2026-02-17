/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "ABC 13 22 1\n",
            "DEF 10 20 30\n",
            "GHI 11 2 2\n",
            "JKL 20 20 20\n",
        ))
        .assert();
    assert.success().stdout(concat!("JKL\n", "DEF\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4\n",
            "JKL 20 20 20\n",
            "GHI 20 20 20\n",
            "DEF 20 20 20\n",
            "ABC 20 20 20\n",
        ))
        .assert();
    assert.success().stdout(concat!("ABC\n", "DEF\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n", "JKL 20 20 20\n",)).assert();
    assert.success().stdout(concat!("JKL\n",));
}

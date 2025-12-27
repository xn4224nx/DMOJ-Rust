/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1 2\n",
            "###########\n",
            "#....#****#\n",
            "#....#****#\n",
            "#....#....#\n",
            "#....#....#\n",
            "###########\n",
        ))
        .assert();
    assert.success().stdout(concat!("1 0 1 0 0\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "2 3\n",
            "################\n",
            "#****#****#****#\n",
            "#****#****#****#\n",
            "#****#....#****#\n",
            "#....#....#****#\n",
            "################\n",
            "#....#****#****#\n",
            "#....#****#....#\n",
            "#....#....#....#\n",
            "#....#....#....#\n",
            "################\n",
        ))
        .assert();
    assert.success().stdout(concat!("1 1 2 1 1\n",));
}

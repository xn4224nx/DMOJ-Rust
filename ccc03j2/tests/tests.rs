/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("100\n", "15\n", "195\n", "11\n", "0\n",))
        .assert();
    assert.success().stdout(concat!(
        "Minimum perimeter is 40 with dimensions 10 x 10\n",
        "Minimum perimeter is 16 with dimensions 3 x 5\n",
        "Minimum perimeter is 56 with dimensions 13 x 15\n",
        "Minimum perimeter is 24 with dimensions 1 x 11\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("100\n", "0\n",)).assert();
    assert
        .success()
        .stdout(concat!("Minimum perimeter is 40 with dimensions 10 x 10\n",));
}

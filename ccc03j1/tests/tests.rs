/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("4\n", "3\n", "2\n",)).assert();
    assert.success().stdout(concat!(
        "*   *   *\n",
        "*   *   *\n",
        "*   *   *\n",
        "*   *   *\n",
        "*********\n",
        "    *\n",
        "    *\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("3\n", "2\n", "4\n",)).assert();
    assert.success().stdout(concat!(
        "*  *  *\n",
        "*  *  *\n",
        "*  *  *\n",
        "*******\n",
        "   *\n",
        "   *\n",
        "   *\n",
        "   *\n",
    ));
}

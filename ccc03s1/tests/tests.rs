/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "9\n", "11\n", "12\n", "7\n", "3\n", "5\n", "10\n", "9\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "You are now on square 10\n",
        "You are now on square 21\n",
        "You are now on square 33\n",
        "You are now on square 64\n",
        "You are now on square 86\n",
        "You are now on square 91\n",
        "You are now on square 91\n",
        "You are now on square 100\n",
        "You Win!\n",
    ));
}

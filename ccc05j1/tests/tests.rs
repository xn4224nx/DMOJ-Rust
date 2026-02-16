/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("251\n", "10\n", "60\n",)).assert();
    assert.success().stdout(concat!(
        "Plan A costs 51.25\n",
        "Plan B costs 18.95\n",
        "Plan B is cheapest.\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("162\n", "61\n", "66\n",)).assert();
    assert.success().stdout(concat!(
        "Plan A costs 37.85\n",
        "Plan B costs 37.85\n",
        "Plan A and B are the same price.\n",
    ));
}

/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "R\n", "QUEEN\n", "R\n", "FOURTH\n", "R\n", "SCHOOL\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "Turn LEFT onto FOURTH street.\n",
        "Turn LEFT onto QUEEN street.\n",
        "Turn LEFT into your HOME.\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("L\n", "MAIN\n", "R\n", "SCHOOL\n",))
        .assert();
    assert.success().stdout(concat!(
        "Turn LEFT onto MAIN street.\n",
        "Turn RIGHT into your HOME.\n",
    ));
}

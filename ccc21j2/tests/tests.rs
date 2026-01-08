/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3\n",
            "Ahmed\n",
            "300\n",
            "Suzanne\n",
            "500\n",
            "Ivona\n",
            "450\n",
        ))
        .assert();
    assert.success().stdout(concat!("Suzanne\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("2\n", "Ijeoma\n", "20\n", "Goor\n", "20\n",))
        .assert();
    assert.success().stdout(concat!("Ijeoma\n",));
}

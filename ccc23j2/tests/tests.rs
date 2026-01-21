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
            "Poblano\n",
            "Cayenne\n",
            "Thai\n",
            "Poblano\n",
        ))
        .assert();
    assert.success().stdout(concat!("118000\n",));
}

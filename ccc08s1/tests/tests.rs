/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "Saskatoon -20\n",
            "Toronto -2\n",
            "Winnipeg -40\n",
            "Vancouver 8\n",
            "Halifax 0\n",
            "Montreal -4\n",
            "Waterloo -3\n",
        ))
        .assert();
    assert.success().stdout(concat!("Winnipeg\n",));
}

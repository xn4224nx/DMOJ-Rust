/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("CCC\n", ":-)\n", "SQL\n", "TTYL\n",))
        .assert();
    assert.success().stdout(concat!(
        "Canadian Computing Competition\n",
        "I'm happy\n",
        "SQL\n",
        "talk to you later\n",
    ));
}

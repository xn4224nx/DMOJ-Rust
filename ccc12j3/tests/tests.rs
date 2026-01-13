/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("3\n",)).assert();
    assert.success().stdout(concat!(
        "***xxx***\n",
        "***xxx***\n",
        "***xxx***\n",
        "   xxxxxx\n",
        "   xxxxxx\n",
        "   xxxxxx\n",
        "***   ***\n",
        "***   ***\n",
        "***   ***\n",
    ));
}

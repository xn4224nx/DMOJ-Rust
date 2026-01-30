/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "5\n",
            "1933 10 29\n",
            "1989 2 28\n",
            "1961 11 23\n",
            "1999 12 31\n",
            "1989 2 27\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("Yes\n", "No\n", "Yes\n", "No\n", "Yes\n",));
}

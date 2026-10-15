/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("1\n", "2\n", "3\n", "4\n", "3\n",))
        .assert();
    assert.success().stdout(concat!(
        "# of PINK is 0 # of GREEN is 0 # of RED is 1 # of ORANGE is 0\n",
        "# of PINK is 1 # of GREEN is 1 # of RED is 0 # of ORANGE is 0\n",
        "# of PINK is 3 # of GREEN is 0 # of RED is 0 # of ORANGE is 0\n",
        "Total combinations is 3.\n",
        "Minimum number of tickets to print is 1.\n",
    ));
}

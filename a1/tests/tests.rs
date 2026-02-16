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
            "4 MISSPELL\n",
            "1 PROGRAMMING\n",
            "7 CONTEST\n",
            "3 BALLOON\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "1 MISPELL\n",
        "2 ROGRAMMING\n",
        "3 CONTES\n",
        "4 BALOON\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("1\n", "11 MISSPELL THE WORD TODAY\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("1 MISSPELL TE WORD TODAY\n",));
}

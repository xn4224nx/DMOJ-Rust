/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "105\n",
            "14\n",
            "16\n",
            "IIIIIIIIIIIIIIII\n",
            "I......I.......I\n",
            "I......III.....I\n",
            "I........I.....I\n",
            "I........IIIIIII\n",
            "IIIIIIIIII.....I\n",
            "I.I......I.....I\n",
            "III..III.I.....I\n",
            "I....I.IIIII...I\n",
            "I....I.....III.I\n",
            "I....I.......I.I\n",
            "I....I.....III.I\n",
            "I....I.....I...I\n",
            "IIIIIIIIIIIIIIII\n",
        ))
        .assert();
    assert
        .success()
        .stdout(concat!("4 rooms, 1 square metre(s) left over\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!("13\n", "2\n", "3\n", ".I.\n", ".I.\n",))
        .assert();
    assert
        .success()
        .stdout(concat!("2 rooms, 9 square metre(s) left over\n",));
}

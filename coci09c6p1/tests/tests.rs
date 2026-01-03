/*
 * Tests for main.rs
 */

use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "10 10\n",
            "S.....111F\n",
            "S....222.F\n",
            "S...333..F\n",
            "S..444...F\n",
            "S.555....F\n",
            "S666.....F\n",
            "S.777....F\n",
            "S..888...F\n",
            "S...999..F\n",
            "S........F\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "1\n", "2\n", "3\n", "4\n", "5\n", "6\n", "5\n", "4\n", "3\n",
    ));
}
#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "10 15\n",
            "S..........222F\n",
            "S.....111.....F\n",
            "S...333.......F\n",
            "S...555.......F\n",
            "S.......444...F\n",
            "S.............F\n",
            "S......777....F\n",
            "S..888........F\n",
            "S........999..F\n",
            "S...666.......F\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "5\n", "1\n", "6\n", "3\n", "6\n", "6\n", "4\n", "7\n", "2\n",
    ));
}

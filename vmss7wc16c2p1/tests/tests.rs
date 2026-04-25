/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("1\n",)).assert();
    assert.success().stdout(concat!(
        "GGGGG\n", "G....\n", "G..GG\n", "G...G\n", "GGGGG\n",
    ));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd.write_stdin(concat!("3\n",)).assert();
    assert.success().stdout(concat!(
        "GGGGGGGGGGGGGGG\n",
        "GGGGGGGGGGGGGGG\n",
        "GGGGGGGGGGGGGGG\n",
        "GGG............\n",
        "GGG............\n",
        "GGG............\n",
        "GGG......GGGGGG\n",
        "GGG......GGGGGG\n",
        "GGG......GGGGGG\n",
        "GGG.........GGG\n",
        "GGG.........GGG\n",
        "GGG.........GGG\n",
        "GGGGGGGGGGGGGGG\n",
        "GGGGGGGGGGGGGGG\n",
        "GGGGGGGGGGGGGGG\n",
    ));
}

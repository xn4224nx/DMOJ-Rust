/*
 * Tests for main.rs
 */

#![allow(deprecated)]
use assert_cmd::{Command, pkg_name};

#[test]
fn full_program_exp0() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "1\n",
            "One plus one is small\n",
            "one hundred plus one is not\n",
            "you might be very tall\n",
            "but summer is not\n",
        ))
        .assert();
    assert.success().stdout(concat!("cross\n",));
}

#[test]
fn full_program_exp1() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "2\n",
            "I say to you boo\n",
            "You say boohoo\n",
            "I cry too\n",
            "It is too much foo\n",
            "Your teacher has to mark\n",
            "and mark and mark and teach\n",
            "To do well on this contest you have to reach\n",
            "for everything with a lark\n",
        ))
        .assert();
    assert.success().stdout(concat!("perfect\n", "shell\n",));
}

#[test]
fn full_program_exp2() {
    let mut cmd = Command::cargo_bin(pkg_name!()).unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "2\n",
            "It seems though\n",
            "that without some dough\n",
            "creating such a bash\n",
            "is a weighty in terms of cash\n",
            "But how I see\n",
            "the problem so fair\n",
            "is to write subtle verse\n",
            "with hardly a rhyme\n",
        ))
        .assert();
    assert.success().stdout(concat!("even\n", "free\n",));
}

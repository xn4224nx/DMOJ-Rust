/*
 * Tests for main.rs
 */

#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod ecoo19r2p1;
use ecoo19r2p1::strip_email;

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ecoo19r2p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3\n",
            "foo@bar.com\n",
            "fO.o+baz123@bAR.com\n",
            "foo@bar..com\n",
            "3\n",
            "c++@foo.com\n",
            "c...@Foo.com\n",
            ".c+c@FOO.COM\n",
            "3\n",
            "foo@bar.com\n",
            "fO.o+baz123@bAR.com\n",
            "foo@bar..com\n",
            "3\n",
            "c++@foo.com\n",
            "c...@Foo.com\n",
            ".c+c@FOO.COM\n",
            "3\n",
            "foo@bar.com\n",
            "fO.o+baz123@bAR.com\n",
            "foo@bar..com\n",
            "3\n",
            "c++@foo.com\n",
            "c...@Foo.com\n",
            ".c+c@FOO.COM\n",
            "3\n",
            "foo@bar.com\n",
            "fO.o+baz123@bAR.com\n",
            "foo@bar..com\n",
            "3\n",
            "c++@foo.com\n",
            "c...@Foo.com\n",
            ".c+c@FOO.COM\n",
            "3\n",
            "foo@bar.com\n",
            "fO.o+baz123@bAR.com\n",
            "foo@bar..com\n",
            "3\n",
            "c++@foo.com\n",
            "c...@Foo.com\n",
            ".c+c@FOO.COM\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "2\n", "1\n", "2\n", "1\n", "2\n", "1\n", "2\n", "1\n", "2\n", "1\n",
    ));
}

#[test]
fn string_email_exp01() {
    assert_eq!(
        strip_email(&String::from("foo@bar.com")),
        String::from("foo@bar.com")
    );
}

#[test]
fn string_email_exp02() {
    assert_eq!(
        strip_email(&String::from("fO.o+baz123@bAR.com")),
        String::from("foo@bar.com")
    );
}

#[test]
fn string_email_exp03() {
    assert_eq!(
        strip_email(&String::from("foo@bar..com")),
        String::from("foo@bar..com")
    );
}

#[test]
fn string_email_exp04() {
    assert_eq!(
        strip_email(&String::from("c++@foo.com")),
        String::from("c@foo.com")
    );
}

#[test]
fn string_email_exp05() {
    assert_eq!(
        strip_email(&String::from("c...@Foo.com")),
        String::from("c@foo.com")
    );
}

#[test]
fn string_email_exp06() {
    assert_eq!(
        strip_email(&String::from(".c+c@FOO.COM")),
        String::from("c@foo.com")
    );
}

#[test]
fn string_email_exp07() {
    assert_eq!(
        strip_email(&String::from("FOO@BAR.COM")),
        String::from("foo@bar.com")
    );
}

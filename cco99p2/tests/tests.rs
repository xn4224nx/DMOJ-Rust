/*
 * Tests for main.rs
 */

#[path = "../src/main.rs"]
mod cco99p2;
use assert_cmd::Command;
use cco99p2::ordinal_display;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("cco99p2").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3\n", "7 2\n", "the\n", "brown\n", "the\n", "fox\n", "red\n", "the\n", "red\n",
            "1 3\n", "the\n", "2 1\n", "the\n", "wash\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "2nd most common word(s):\n",
        "red\n",
        "\n",
        "3rd most common word(s):\n",
        "\n",
        "1st most common word(s):\n",
        "the\n",
        "wash\n",
    ));
}

#[test]
fn ordinal_str_exp01() {
    assert_eq!(ordinal_display(&1), "st");
}

#[test]
fn ordinal_str_exp02() {
    assert_eq!(ordinal_display(&2), "nd");
}

#[test]
fn ordinal_str_exp03() {
    assert_eq!(ordinal_display(&3), "rd");
}

#[test]
fn ordinal_str_exp04() {
    for num in 4..21 {
        assert_eq!(ordinal_display(&num), "th");
    }
}

#[test]
fn ordinal_str_exp05() {
    assert_eq!(ordinal_display(&21), "st");
}

#[test]
fn ordinal_str_exp06() {
    assert_eq!(ordinal_display(&22), "nd");
}

#[test]
fn ordinal_str_exp07() {
    assert_eq!(ordinal_display(&23), "rd");
}

#[test]
fn ordinal_str_exp08() {
    assert_eq!(ordinal_display(&24), "th");
}

#[test]
fn ordinal_str_exp09() {
    assert_eq!(ordinal_display(&1013), "th");
}

#[test]
fn ordinal_str_exp10() {
    assert_eq!(ordinal_display(&122), "nd");
}

#[test]
fn ordinal_str_exp11() {
    assert_eq!(ordinal_display(&56), "th");
}

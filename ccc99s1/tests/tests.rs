/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc99s1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "three\n", "seven\n", "queen\n", "eight\n", "five\n", "ten\n", "king\n", "eight\n",
            "jack\n", "queen\n", "six\n", "queen\n", "jack\n", "eight\n", "seven\n", "three\n",
            "ten\n", "four\n", "king\n", "nine\n", "six\n", "seven\n", "ace\n", "four\n", "jack\n",
            "ace\n", "ten\n", "nine\n", "ten\n", "queen\n", "ace\n", "king\n", "seven\n", "two\n",
            "five\n", "two\n", "five\n", "nine\n", "three\n", "king\n", "six\n", "eight\n",
            "jack\n", "six\n", "five\n", "four\n", "two\n", "ace\n", "four\n", "three\n", "two\n",
            "nine\n",
        ))
        .assert();
    assert.success().stdout(concat!(
        "Player A scores 2 point(s).\n",
        "Player A scores 1 point(s).\n",
        "Player A scores 3 point(s).\n",
        "Player B scores 3 point(s).\n",
        "Player A scores 1 point(s).\n",
        "Player B scores 4 point(s).\n",
        "Player A: 7 point(s).\n",
        "Player B: 7 point(s).\n",
    ));
}

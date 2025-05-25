/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ecoo17r3p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "4 5\n",
            "4 3 2 4\n",
            "3 3 2 1\n",
            "8 2 4 1\n",
            "2 2 4 3\n",
            "9 3 2 3\n",
            "4 2\n",
            "4 4 4 1\n",
            "1 1 3 4\n",
            "4 5\n",
            "4 3 2 4\n",
            "3 3 2 1\n",
            "8 2 4 1\n",
            "2 2 4 3\n",
            "9 3 2 3\n",
            "4 2\n",
            "4 4 4 1\n",
            "1 1 3 4\n",
            "4 5\n",
            "4 3 2 4\n",
            "3 3 2 1\n",
            "8 2 4 1\n",
            "2 2 4 3\n",
            "9 3 2 3\n",
            "4 2\n",
            "4 4 4 1\n",
            "1 1 3 4\n",
            "4 5\n",
            "4 3 2 4\n",
            "3 3 2 1\n",
            "8 2 4 1\n",
            "2 2 4 3\n",
            "9 3 2 3\n",
            "4 2\n",
            "4 4 4 1\n",
            "1 1 3 4\n",
            "4 5\n",
            "4 3 2 4\n",
            "3 3 2 1\n",
            "8 2 4 1\n",
            "2 2 4 3\n",
            "9 3 2 3\n",
            "4 2\n",
            "4 4 4 1\n",
            "1 1 3 4\n",
        ))
        .assert();
    assert.success().stdout("4\n1\n4\n1\n4\n1\n4\n1\n4\n1\n");
}

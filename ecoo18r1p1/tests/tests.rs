/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ecoo18r1p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3 5\n", "E\n", "B\n", "E\n", "B\n", "E\n", "2 4\n", "B\n", "E\n", "E\n", "E\n",
            "3 5\n", "E\n", "B\n", "E\n", "B\n", "E\n", "2 4\n", "B\n", "E\n", "E\n", "E\n",
            "3 5\n", "E\n", "B\n", "E\n", "B\n", "E\n", "2 4\n", "B\n", "E\n", "E\n", "E\n",
            "3 5\n", "E\n", "B\n", "E\n", "B\n", "E\n", "2 4\n", "B\n", "E\n", "E\n", "E\n",
            "3 5\n", "E\n", "B\n", "E\n", "B\n", "E\n", "2 4\n", "B\n", "E\n", "E\n", "E\n",
        ))
        .assert();
    assert.success().stdout("2\n0\n2\n0\n2\n0\n2\n0\n2\n0\n");
}

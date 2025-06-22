/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("dmopc19c5p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "3 4\n",
            "chalk\n",
            "cheese\n",
            "charger\n",
            "1\n",
            "cheese\n",
            "2\n",
            "coins\n",
            "cash\n",
            "3\n",
            "charger\n",
            "chalk\n",
            "caffeine\n",
            "3\n",
            "cheese\n",
            "charger\n",
            "cheese\n",
        ))
        .assert();
    assert.success().stdout(concat!("2\n",));
}

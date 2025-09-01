/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("dmopc17c5p1").unwrap();
    let assert = cmd.write_stdin(concat!("y0105w49\n",)).assert();
    assert.success().stdout(concat!("yOlOSwAg\n",));
}

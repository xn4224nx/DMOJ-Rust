/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ccc11s1").unwrap();
    let assert = cmd
        .write_stdin("3\nThe red cat sat on the mat.\nWhy are you so sad cat?\nDon't ask that.\n")
        .assert();
    assert.success().stdout("English\n");
}

#[test]
fn exp_02() {
    let mut cmd = Command::cargo_bin("ccc11s1").unwrap();
    let assert = cmd
        .write_stdin(
            "3\nLorsque j'avais six ans j'ai vu, une fois,\nune magnifique image,\ndans un livre\n",
        )
        .assert();
    assert.success().stdout("French\n");
}

#[test]
fn exp_03() {
    let mut cmd = Command::cargo_bin("ccc11s1").unwrap();
    let assert = cmd.write_stdin("4\nSi je discernais ta voix encore\nConnaissant ce coeur qui doute,\nTu me dirais de tirer un trait\nQuoi que partir me coute.\n").assert();
    assert.success().stdout("English\n");
}

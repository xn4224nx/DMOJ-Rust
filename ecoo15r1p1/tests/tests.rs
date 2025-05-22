/*
 * Tests for main.rs
 */

use assert_cmd::Command;

#[test]
fn exp_01() {
    let mut cmd = Command::cargo_bin("ecoo15r1p1").unwrap();
    let assert = cmd
        .write_stdin(concat!(
            "red\nbrown\nbrown\nviolet\nblue\npink\nblue\nblue\n",
            "pink\nbrown\nyellow\nbrown\npink\nviolet\ngreen\n",
            "yellow\nred\norange\norange\nblue\nbrown\npink\n",
            "red\nred\nred\nbrown\norange\norange\ngreen\nred\n",
            "orange\nviolet\nblue\npink\nyellow\npink\nbrown\norange\n",
            "green\nred\nblue\nyellow\ngreen\norange\nbrown\norange\n",
            "pink\nviolet\nbrown\nred\nend of box\n",
            "red\nbrown\nbrown\nviolet\nblue\npink\nblue\nblue\n",
            "pink\nbrown\nyellow\nbrown\npink\nviolet\ngreen\n",
            "yellow\nred\norange\norange\nblue\nbrown\npink\n",
            "red\nred\nred\nbrown\norange\norange\ngreen\nred\n",
            "orange\nviolet\nblue\npink\nyellow\npink\nbrown\norange\n",
            "green\nred\nblue\nyellow\ngreen\norange\nbrown\norange\n",
            "pink\nviolet\nbrown\nred\nend of box\n",
            "red\nbrown\nbrown\nviolet\nblue\npink\nblue\nblue\n",
            "pink\nbrown\nyellow\nbrown\npink\nviolet\ngreen\n",
            "yellow\nred\norange\norange\nblue\nbrown\npink\n",
            "red\nred\nred\nbrown\norange\norange\ngreen\nred\n",
            "orange\nviolet\nblue\npink\nyellow\npink\nbrown\norange\n",
            "green\nred\nblue\nyellow\ngreen\norange\nbrown\norange\n",
            "pink\nviolet\nbrown\nred\nend of box\n",
            "red\nbrown\nbrown\nviolet\nblue\npink\nblue\nblue\n",
            "pink\nbrown\nyellow\nbrown\npink\nviolet\ngreen\n",
            "yellow\nred\norange\norange\nblue\nbrown\npink\n",
            "red\nred\nred\nbrown\norange\norange\ngreen\nred\n",
            "orange\nviolet\nblue\npink\nyellow\npink\nbrown\norange\n",
            "green\nred\nblue\nyellow\ngreen\norange\nbrown\norange\n",
            "pink\nviolet\nbrown\nred\nend of box\n",
            "red\nbrown\nbrown\nviolet\nblue\npink\nblue\nblue\n",
            "pink\nbrown\nyellow\nbrown\npink\nviolet\ngreen\n",
            "yellow\nred\norange\norange\nblue\nbrown\npink\n",
            "red\nred\nred\nbrown\norange\norange\ngreen\nred\n",
            "orange\nviolet\nblue\npink\nyellow\npink\nbrown\norange\n",
            "green\nred\nblue\nyellow\ngreen\norange\nbrown\norange\n",
            "pink\nviolet\nbrown\nred\nend of box\n",
            "red\nbrown\nbrown\nviolet\nblue\npink\nblue\nblue\n",
            "pink\nbrown\nyellow\nbrown\npink\nviolet\ngreen\n",
            "yellow\nred\norange\norange\nblue\nbrown\npink\n",
            "red\nred\nred\nbrown\norange\norange\ngreen\nred\n",
            "orange\nviolet\nblue\npink\nyellow\npink\nbrown\norange\n",
            "green\nred\nblue\nyellow\ngreen\norange\nbrown\norange\n",
            "pink\nviolet\nbrown\nred\nend of box\n",
            "red\nbrown\nbrown\nviolet\nblue\npink\nblue\nblue\n",
            "pink\nbrown\nyellow\nbrown\npink\nviolet\ngreen\n",
            "yellow\nred\norange\norange\nblue\nbrown\npink\n",
            "red\nred\nred\nbrown\norange\norange\ngreen\nred\n",
            "orange\nviolet\nblue\npink\nyellow\npink\nbrown\norange\n",
            "green\nred\nblue\nyellow\ngreen\norange\nbrown\norange\n",
            "pink\nviolet\nbrown\nred\nend of box\n",
            "red\nbrown\nbrown\nviolet\nblue\npink\nblue\nblue\n",
            "pink\nbrown\nyellow\nbrown\npink\nviolet\ngreen\n",
            "yellow\nred\norange\norange\nblue\nbrown\npink\n",
            "red\nred\nred\nbrown\norange\norange\ngreen\nred\n",
            "orange\nviolet\nblue\npink\nyellow\npink\nbrown\norange\n",
            "green\nred\nblue\nyellow\ngreen\norange\nbrown\norange\n",
            "pink\nviolet\nbrown\nred\nend of box\n",
            "red\nbrown\nbrown\nviolet\nblue\npink\nblue\nblue\n",
            "pink\nbrown\nyellow\nbrown\npink\nviolet\ngreen\n",
            "yellow\nred\norange\norange\nblue\nbrown\npink\n",
            "red\nred\nred\nbrown\norange\norange\ngreen\nred\n",
            "orange\nviolet\nblue\npink\nyellow\npink\nbrown\norange\n",
            "green\nred\nblue\nyellow\ngreen\norange\nbrown\norange\n",
            "pink\nviolet\nbrown\nred\nend of box\n",
            "red\nbrown\nbrown\nviolet\nblue\npink\nblue\nblue\n",
            "pink\nbrown\nyellow\nbrown\npink\nviolet\ngreen\n",
            "yellow\nred\norange\norange\nblue\nbrown\npink\n",
            "red\nred\nred\nbrown\norange\norange\ngreen\nred\n",
            "orange\nviolet\nblue\npink\nyellow\npink\nbrown\norange\n",
            "green\nred\nblue\nyellow\ngreen\norange\nbrown\norange\n",
            "pink\nviolet\nbrown\nred\nend of box\n",
        ))
        .assert();
    assert
        .success()
        .stdout("245\n245\n245\n245\n245\n245\n245\n245\n245\n245\n");
}

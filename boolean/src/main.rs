/*
 * Boolean
 * https://dmoj.ca/problem/boolean
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Parse the chain of commands. */
    let commd = buffer.trim_end().split_whitespace().collect::<Vec<&str>>();

    println!(
        "{}",
        if commd.len() % 2 == 0 && commd[commd.len() - 1] == "True" {
            "False"
        } else if commd.len() % 2 == 0 && commd[commd.len() - 1] == "False" {
            "True"
        } else if commd.len() % 2 != 0 && commd[commd.len() - 1] == "True" {
            "True"
        } else {
            "False"
        }
    );
}

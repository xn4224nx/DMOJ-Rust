/*
 * WC '17 Contest 2 J1 - The Rings of Power
 * https://dmoj.ca/problem/wc17c2j1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of rings. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* How many rings are there? */
    println!(
        "{}",
        1 + buffer
            .trim_end()
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .sum::<usize>()
    );
}

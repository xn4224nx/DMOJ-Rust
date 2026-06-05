/*
 * WOSS Dual Olympiad 2023 J1: Minimum Price
 * https://dmoj.ca/problem/wossoly23j1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the problem data. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Find the minimum number. */
    println!(
        "{}",
        buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .min()
            .unwrap()
    );
}

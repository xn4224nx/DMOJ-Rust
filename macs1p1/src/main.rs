/*
 * Max's Anger Contest Series 1 P1 - Hunger Bar
 * https://dmoj.ca/problem/macs1p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the input data. */
    for _ in 0..3 {
        std::io::stdin().read_line(&mut buffer).unwrap();
    }

    /* Parse the input data. */
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!(
        "{}",
        std::cmp::min(data[0], data[1]).saturating_sub(data[2])
    );
}

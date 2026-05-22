/*
 * COCI '09 Contest 2 #1 Faktor
 * https://dmoj.ca/problem/coci09c2p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of articles to publish and the impact factor. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let in_data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* What is the minimal number of scientists you need to bribe? */
    println!("{}", in_data[0] * (in_data[1] - 1) + 1)
}

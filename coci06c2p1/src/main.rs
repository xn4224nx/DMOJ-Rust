/*
 * COCI '06 Contest 2 #1 R2
 * https://dmoj.ca/problem/coci06c2p1
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("{}", 2 * data[1] - data[0]);
}

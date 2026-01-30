/*
 * CCC '12 S1 - Don't pass me the ball!
 * https://dmoj.ca/problem/ccc12s1
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num = buffer.trim_end().parse::<usize>().unwrap();
    println!(
        "{}",
        (num.saturating_sub(1)) * (num.saturating_sub(2)) * (num.saturating_sub(3)) / 6
    );
}

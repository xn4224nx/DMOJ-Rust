/*
 * An Animal Contest 2 P0 - Koala Matchmaking
 * https://dmoj.ca/problem/aac2p0
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    println!("{}", buffer.trim_end().parse::<usize>().unwrap() - 1);
}

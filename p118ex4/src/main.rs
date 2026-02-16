/*
 * BlueBook - Times Tables
 * https://dmoj.ca/problem/p118ex4
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let factor = buffer.trim_end().parse::<usize>().unwrap();
    for val in 1..=factor {
        println!("{} X {} = {}", factor, val, factor * val);
    }
}

/*
 * OTHS Coding Competition 4 P1 - Square Root Decomposition
 * https://dmoj.ca/problem/othscc4p1
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    println!("{}", 8 * buffer.trim().parse::<usize>().unwrap());
}

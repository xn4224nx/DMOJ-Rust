/*
 * Amplitude Hackathon Winter '24 Problem 1 - Twila's Walk
 * https://dmoj.ca/problem/ampl2024wp1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the length of the walk. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let walk_len = buffer.trim_end().parse::<usize>().unwrap();

    /* Print the resultant increase in happiness. */
    println!("{}", walk_len.pow(2));
}

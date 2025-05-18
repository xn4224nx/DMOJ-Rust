/*
 * COCI '13 Contest 3 #1 Riječi
 * https://dmoj.ca/problem/coci13c3p1
 */

fn main() {
    let mut buffer = String::new();
    let mut old_a = 1;
    let mut old_b = 0;

    /* Read the number of button presses. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let presses = buffer.trim().parse::<usize>().unwrap();

    /* Simultate button presses. */
    for _ in 0..presses {
        (old_a, old_b) = (old_b, old_a + old_b);
    }
    println!("{} {}", old_a, old_b);
}

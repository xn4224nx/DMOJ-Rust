/*
 * PIB '20 P1 - Marcus and Head Bonks
 * https://dmoj.ca/problem/pib20p1
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Read the numbers and count the postive ones. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    println!(
        "{}",
        buffer
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .filter(|x| *x > 0)
            .count()
    );
}

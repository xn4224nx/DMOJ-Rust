/*
 * COCI '09 Contest 4 #1 Autori
 * https://dmoj.ca/problem/coci09c4p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the names. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Only print the uppercase characters. */
    println!(
        "{}",
        buffer
            .chars()
            .filter(|x| x.is_ascii_uppercase())
            .collect::<String>()
    );
}

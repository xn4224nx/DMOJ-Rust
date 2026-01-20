/*
 * CCC '12 J4 - Big Bang Secrets
 * https://dmoj.ca/problem/ccc12j4
 */

fn main() {
    let mut buffer = String::new();

    /* Read the encryption key. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let key = buffer.trim_end().parse::<u8>().unwrap();

    /* Read the encyrpted message. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let encr_msg: Vec<u8> = buffer.trim_end().as_bytes().to_vec();

    /* Decrypt the message */
    println!(
        "{}",
        encr_msg
            .into_iter()
            .enumerate()
            .map(
                |(idx, letter)| (((letter - ('A' as u8) + (26 - key - 3 * (idx as u8 + 1))) % 26)
                    + ('A' as u8)) as char
            )
            .collect::<String>()
    );
}

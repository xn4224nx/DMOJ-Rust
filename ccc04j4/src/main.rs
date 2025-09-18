/*
 * CCC '04 J4 - Simple Encryption
 * https://dmoj.ca/problem/ccc04j4
 */

fn main() {
    let mut buffer = String::new();

    /* Read the key and strip out non-letter characters. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let key = buffer
        .chars()
        .filter(|x| x.is_uppercase())
        .collect::<Vec<char>>();

    /* Read the message and strip out non-letter characters. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let msg = buffer
        .chars()
        .filter(|x| x.is_uppercase())
        .collect::<Vec<char>>();

    println!("{}", encrypt(&msg, &key));
}

/// Key based Caesar shift encryption
fn encrypt(message: &Vec<char>, key: &Vec<char>) -> String {
    let mut key_idx = 0;
    let mut msg_idx = 0;
    let mut cipher_txt = String::with_capacity(message.len());

    /* Encrypt char by char */
    while msg_idx < message.len() {
        cipher_txt.push(
            ('A' as u8
                + ((message[msg_idx] as u8 - 'A' as u8 + key[key_idx] as u8 - 'A' as u8) % 26))
                as char,
        );
        msg_idx += 1;
        key_idx = (key_idx + 1) % key.len();
    }
    return cipher_txt;
}

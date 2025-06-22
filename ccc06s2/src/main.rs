/*
 * CCC '06 S2 - Attack of the CipherTexts
 * https://dmoj.ca/problem/ccc06s2
 */

use std::collections::HashSet;

fn main() {
    let mut unseen_chars: HashSet<char> = HashSet::from([
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' ',
    ]);
    let mut decryptor: Vec<char> = vec!['.'; 27];
    let mut buffer = String::new();

    /* Read the plain text. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.pop();
    let plain: Vec<char> = buffer.chars().collect();

    /* Read the cypher text and create decryption lookup.*/
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.pop();

    /* Set the decryptor. */
    for (enc_idx, enc_char) in buffer.chars().enumerate() {
        decryptor[char_idx(enc_char)] = plain[enc_idx];

        /* Make a record of remaining chars. */
        unseen_chars.remove(&plain[enc_idx]);
    }

    /* If all but one of the chars has been detected we can infer the last one. */
    if unseen_chars.len() == 1 {
        for ele_idx in 0..decryptor.len() {
            if decryptor[ele_idx] == '.' {
                decryptor[ele_idx] = unseen_chars.drain().next().expect("No remaining char.");
                break;
            }
        }
    }

    /* Read the encrypted message and create the decrypted one. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.pop();
    println!(
        "{}",
        buffer
            .chars()
            .map(|x| decryptor[char_idx(x)])
            .collect::<String>()
    );
}

/// Convert the encoded characer to the index it will be in the decryptor.
fn char_idx(enc_val: char) -> usize {
    return if enc_val == ' ' {
        26
    } else if enc_val.is_uppercase() {
        enc_val as usize - 'A' as usize
    } else {
        panic!("Unrecognised character encountered!");
    };
}

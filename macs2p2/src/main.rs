/*
 * Max's Anger Contest Series 2 P2 - Password Anger
 * https://dmoj.ca/problem/macs2p2
 */

const MIN_LETTER: usize = 97;
const MAX_LETTER: usize = 122;
const HASH_MULTI: usize = 13;

fn main() {
    let mut buffer = String::new();
    let mut hash_collisions = 0;

    /* Read the problem constants. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let pass_len = buffer.trim_end().parse::<usize>().unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let pass_hash_to_find = buffer.trim_end().parse::<usize>().unwrap();

    /* Iterate over every possible password and count the number with the hash. */
    let mut curr_pass = vec![MIN_LETTER; pass_len];
    for _ in 0..(1 + MAX_LETTER - MIN_LETTER).pow(pass_len as u32) {
        if hash_pass(&curr_pass) == pass_hash_to_find {
            hash_collisions += 1;
        }

        /* Iterate onto the next password. */
        for d_idx in 0..curr_pass.len() {
            if curr_pass[d_idx] == MAX_LETTER {
                curr_pass[d_idx] = MIN_LETTER;
            } else {
                curr_pass[d_idx] += 1;
                break;
            }
        }
    }
    println!("{}", hash_collisions);
}

fn hash_pass(msg: &Vec<usize>) -> usize {
    let mut fhash = 0;
    for idx in 0..msg.len() {
        fhash = fhash * HASH_MULTI + msg[idx] - MIN_LETTER + 1;
    }
    return fhash;
}

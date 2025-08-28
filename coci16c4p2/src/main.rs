/*
 * COCI '16 Contest 4 #2 Kartomat
 * https://dmoj.ca/problem/coci16c4p2
 */

const MAX_DEST_LEN: usize = 100;

fn main() {
    let mut letters_to_show = vec![false; 26];
    let mut buffer = String::new();

    /* How many destinations are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_dests = buffer.trim().parse::<usize>().unwrap();

    /* Allocate space for the destinations. */
    let mut dests: Vec<Vec<char>> = vec![Vec::with_capacity(MAX_DEST_LEN); num_dests];

    /* Read the destinations in char by char. */
    for dest_idx in 0..num_dests {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        for d_char in buffer.trim().chars() {
            dests[dest_idx].push(d_char)
        }
    }

    /* Read the intial letters */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let init: Vec<char> = buffer.trim().chars().collect();

    /* Find the matching dests and thus the possible next letters. */
    'dest_check: for dest_idx in 0..num_dests {
        if dests[dest_idx].len() <= init.len() {
            continue;
        }

        /* Determine if this is a viable destination. */
        for ch_idx in 0..init.len() {
            if init[ch_idx] != dests[dest_idx][ch_idx] {
                continue 'dest_check;
            }
        }

        /* Save the next letter that can be printed after the inital letters. */
        letters_to_show[dests[dest_idx][init.len()] as usize - 'A' as usize] = true;
    }

    print!("***");

    /* Print the keyboard with only the possible letters */
    for letter_idx in 0..letters_to_show.len() {
        if letters_to_show[letter_idx] {
            print!("{}", (letter_idx as u8 + 'A' as u8) as char)
        } else {
            print!("*")
        }

        /* Put the new lines in. */
        if letter_idx == 4 || letter_idx == 12 || letter_idx == 20 {
            println!();
        }
    }
    println!("***")
}

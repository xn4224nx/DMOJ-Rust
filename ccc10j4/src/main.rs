/*
 * CCC '10 J4 - Global Warming
 * https://dmoj.ca/problem/ccc10j4
 */

fn main() {
    let mut buffer = String::new();

    /* Process sequences until the exit command is given */
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Test for the exit command. */
        if buffer == String::from("0\n") {
            break;
        }

        /* Parse the temperature sequence. */
        let temp_sequence = buffer
            .trim_end()
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        println!("{}", cycle_len(&temp_sequence));
    }
}

/// For a sequence of temperatures determine the length of the cycle of changes.
fn cycle_len(orig_sequence: &Vec<i32>) -> usize {
    let mut changes = Vec::new();

    /* If there is a sequence with length 1, the cycle is length 0. */
    if orig_sequence.is_empty() || orig_sequence.len() == 1 {
        return 0;
    }

    /* Extract the changes in temperature. */
    for idx in 1..orig_sequence.len() {
        changes.push(orig_sequence[idx - 1] - orig_sequence[idx]);
    }

    assert_eq!(orig_sequence.len() - 1, changes.len());

    /* Check for various sequence lengths. */
    'sequence_check: for seq_len in 1..changes.len() {
        /* Check if this sequence is valid by comparing elements. */
        for t_idx in seq_len..changes.len() {
            if changes[t_idx] != changes[t_idx % seq_len] {
                continue 'sequence_check;
            }
        }

        /* If every check passes, this is the sequence length.  */
        return seq_len;
    }

    /* At this point default to the entire sequence. */
    return changes.len();
}

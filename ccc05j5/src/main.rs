/*
 * CCC '05 J5 - Bananas
 * https://dmoj.ca/problem/ccc05j5
 */

fn main() {
    let mut buffer = String::new();
    loop {
        std::io::stdin().read_line(&mut buffer).unwrap();
        if buffer == "X\n".to_string() {
            break;
        }
        println!(
            "{}",
            if is_monkey_word(&buffer.trim_end().chars().collect()) {
                "YES"
            } else {
                "NO"
            }
        );
        buffer.clear();
    }
}

fn is_monkey_word(word: &Vec<char>) -> bool {
    let mut a_seen = false;
    let mut b_s_diff: usize = 0;
    let mut s_count = 0;
    let mut b_count = 0;

    for idx in 0..word.len() {
        if word[idx] == 'A' {
            a_seen = true;
        } else if word[idx] == 'B' {
            b_count += 1;
            b_s_diff += 1;
        } else if word[idx] == 'S' {
            s_count += 1;
            b_s_diff = b_s_diff.saturating_sub(1)
        }

        /* N must be surrounded by other letters. */
        if (idx == 0 || idx == word.len() - 1) && word[idx] == 'N' {
            return false;
        }

        if idx > 0 {
            /* Ensure no A's or N's are next to each other */
            if (word[idx] == 'A' || word[idx] == 'N') && word[idx] == word[idx - 1] {
                return false;
            }

            /* Ensure B and S are not next to each other. */
            if word[idx] == 'B' && word[idx - 1] == 'S'
                || (word[idx - 1] == 'B' && word[idx] == 'S')
            {
                return false;
            }

            /* Ensure that N is not before S. */
            if word[idx - 1] == 'N' && word[idx] == 'S' {
                return false;
            }

            /* Ensure that A is not before B. */
            if word[idx - 1] == 'A' && word[idx] == 'B' {
                return false;
            }
        }
    }

    /* Make sure that:
     *      - The letter A has been seen.
     *      - S has always been precended by a B.
     *      - The count of B and S match.
     */
    return a_seen && b_s_diff == 0 && s_count == b_count;
}

/*
 * CCC '20 J4 - Cyclic Shifts
 * https://dmoj.ca/problem/ccc20j4
 */

fn main() {
    let mut buffer = String::new();

    /* Read the two strings. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Parse them and convert to vectors to enable index comparisons. */
    let shift_data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!(
        "{}",
        if is_str_sub_cycle(&shift_data[0], &shift_data[1]) {
            "yes"
        } else {
            "no"
        }
    );
}

fn is_str_sub_cycle(main_pat: &Vec<char>, sub_pat: &Vec<char>) -> bool {
    'main_ch: for m_idx in 0..main_pat.len() {
        let mut s_idx = 0;

        /* Only start looking from the start of the sub pattern. */
        if sub_pat[s_idx] != main_pat[m_idx] {
            continue 'main_ch;
        } else {
            s_idx += 1;
        }

        /* Check if the sub pattern continues along the main. */
        while s_idx < sub_pat.len() {
            /* Ensure this main index will exist and it matches. */
            if m_idx + s_idx >= main_pat.len() || sub_pat[s_idx] != main_pat[m_idx + s_idx] {
                break;
            } else {
                s_idx += 1;
            }
        }

        /* Check for a sub pattern loopback. */
        while s_idx < sub_pat.len() {
            if m_idx + s_idx < sub_pat.len()
                || sub_pat[s_idx] != main_pat[m_idx + s_idx - sub_pat.len()]
            {
                continue 'main_ch;
            } else {
                s_idx += 1;
            }
        }

        /* The sub pattern has been found. */
        return true;
    }

    /* The subpattern has not been found */
    return false;
}

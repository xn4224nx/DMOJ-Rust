/*
 * Wesley's Anger Contest 3 Problem 3 - Wesley Plays DDR
 * https://dmoj.ca/problem/wac3p3
 */

fn main() {
    let mut total_score = 0;
    let mut buffer = String::new();

    /* Read the dance moves. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let dance_moves: Vec<u8> = buffer.clone().into_bytes();
    buffer.clear();

    /* Read the number of combos. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_combs = buffer.trim().parse::<usize>().unwrap();

    /* Read the combos and the scores. */
    let mut combs: Vec<Vec<u8>> = Vec::with_capacity(num_combs);
    let mut comb_scores: Vec<usize> = Vec::with_capacity(num_combs);

    for _ in 0..num_combs {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let (moves, score) = buffer.trim().split_once(" ").unwrap();

        /* Convert the combo to an array. */
        combs.push(moves.as_bytes().to_owned());

        /* Save the combo score. */
        comb_scores.push(score.parse::<usize>().unwrap());
    }

    /* Check for matching combos */
    let mut move_idx = 0;
    while move_idx < dance_moves.len() {
        let mut comb_score = 0;
        let mut comb_len = 0;

        /* Check if any combo matches. */
        'combo_chk: for cmb_idx in 0..combs.len() {
            for sub_cmb_idx in 0..combs[cmb_idx].len() {
                if combs[cmb_idx][sub_cmb_idx] != dance_moves[move_idx + sub_cmb_idx] {
                    continue 'combo_chk;
                }
            }

            /* Replace the current matched combo with this one. */
            if combs[cmb_idx].len() > comb_len {
                comb_len = combs[cmb_idx].len();
                comb_score = comb_scores[cmb_idx];
            }
        }

        /* If no combo matched move onto the next dance move */
        if comb_len == 0 {
            move_idx += 1;

        /* If any combo matched move to the end of the combo. */
        } else {
            total_score += comb_score;
            move_idx += comb_len;
        }
    }
    println!("{}", total_score + dance_moves.len() - 1);
}

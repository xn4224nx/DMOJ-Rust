/*
 * ECOO '16 R1 P2 - Spindie
 * https://dmoj.ca/problem/ecoo16r1p2
 */

use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();

    /* There will be exactly 10 problems in the input. */
    for _ in 0..10 {
        buffer.clear();

        /* Read the total numbers on the spinner. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let _spinner_size = buffer.trim().parse::<u64>().unwrap();

        /* Read the spinner numbers. */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let spinner = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<HashSet<u64>>();

        /* Read the target numbers. */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let all_targets = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        for final_score in all_targets.into_iter() {
            if is_score_possible(&spinner, final_score) {
                print!("T");
            } else {
                print!("F");
            }
        }
        println!();
    }
}

pub fn is_score_possible(spinner: &HashSet<u64>, score: u64) -> bool {
    let mut inter_values = vec![score];

    /* The game rolls the dice twice. */
    for _ in 0..2 {
        let mut new_inter_values = Vec::new();

        /* Work backward and map the possible values */
        for val in inter_values.drain(..) {
            for spin_num in spinner.iter() {
                /* Could a multiplication be possible? */
                if val % *spin_num == 0 {
                    new_inter_values.push(val / *spin_num);
                }

                /* Could an addition be possible? */
                if val > *spin_num {
                    new_inter_values.push(val - *spin_num);
                }
            }
        }

        /* No values indicates an impossible score. */
        if new_inter_values.is_empty() {
            return false;
        } else {
            inter_values = new_inter_values;
        }
    }

    /* Check if any starting number is on the spinner. */
    for val in inter_values.drain(..) {
        if spinner.contains(&val) {
            return true;
        }
    }
    return false;
}

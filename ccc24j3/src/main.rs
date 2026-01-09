/*
 * CCC '24 J3 - Bronze Count
 * https://dmoj.ca/problem/ccc24j3
 */

const IDX_TO_FIND: usize = 2;

fn main() {
    let mut buffer = String::new();

    /* How many participants were there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_particip = buffer.trim_end().parse::<usize>().unwrap();

    let mut top_scores = vec![0; IDX_TO_FIND + 1];
    let mut t_scores_cnt = vec![0; IDX_TO_FIND + 1];

    /* Read the participants scores. */
    'read_num: for _ in 0..num_particip {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let curr_score = buffer.trim_end().parse::<usize>().unwrap();

        /* Determine if the score already exists in the top scores. */
        for t_idx in 0..=IDX_TO_FIND {
            if top_scores[t_idx] == curr_score {
                t_scores_cnt[t_idx] += 1;
                continue 'read_num;
            }
        }

        /* Determine if one of the numbers should be replaced. */
        for t_idx in 0..=IDX_TO_FIND {
            if curr_score > top_scores[t_idx] {
                top_scores.pop();
                t_scores_cnt.pop();

                /* Insert in the new number. */
                top_scores.insert(t_idx, curr_score);
                t_scores_cnt.insert(t_idx, 1);

                continue 'read_num;
            }
        }
    }

    println!("{} {}", top_scores[IDX_TO_FIND], t_scores_cnt[IDX_TO_FIND]);
}

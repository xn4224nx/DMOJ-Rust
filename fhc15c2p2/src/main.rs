/*
 * Facebook Hacker Cup '15 Round 2 P2 - All Critical
 * https://dmoj.ca/problem/fhc15c2p2
 */

fn main() {
    let num_crit_bars: usize = 20;
    let mut buffer = String::new();

    /* How many cases are there. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_cases = buffer.trim_end().parse::<usize>().unwrap();

    for case_idx in 1..=num_cases {
        buffer.clear();

        /* Read the probability of a critical. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let crit_prob = buffer.trim_end().parse::<f64>().unwrap();

        /* Create intermediatery vectors */
        let mut interm = vec![vec![1.0; num_crit_bars + 1]; num_crit_bars + 1];
        let mut results = vec![1.0; num_crit_bars + 1];
        results[0] = 0.0;

        for idx0 in 1..=num_crit_bars {
            interm[idx0][0] = interm[idx0 - 1][0] * (1.0 - crit_prob);
            interm[idx0][idx0] = interm[idx0 - 1][idx0 - 1] * crit_prob;

            for idx1 in 1..idx0 {
                interm[idx0][idx1] = interm[idx0 - 1][idx1 - 1] * crit_prob
                    + interm[idx0 - 1][idx1] * (1.0 - crit_prob);
            }
        }

        for idx0 in 1..=num_crit_bars {
            for idx1 in 1..=idx0 {
                results[idx0] += interm[idx0][idx1] * results[idx0 - idx1];
            }
            results[idx0] /= 1.0 - interm[idx0][0];
        }

        /* Times you need to play the song to get all critical bars. */
        println!("Case #{}: {:.5}", case_idx, results[num_crit_bars]);
    }
}

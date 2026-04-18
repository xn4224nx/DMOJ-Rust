/*
 * CCO '07 P2 - Snowflakes
 * https://dmoj.ca/problem/cco07p2
 */

use std::collections::HashSet;

const NUM_SNOW_ARMS: usize = 6;

fn main() {
    let mut dupe_seen = false;
    let mut buffer = String::new();

    /* How many snowflakes are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_snw = buffer.trim_end().parse::<usize>().unwrap();

    let mut snow_flakes = HashSet::with_capacity(num_snw * NUM_SNOW_ARMS * 2);

    /* Read the snowflakes arm lengths. */
    for _ in 0..num_snw {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let snw_arms = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Has this snowflake variation been seen before? */
        if !dupe_seen {
            for nm_sflake in normalise_snowflake(&snw_arms) {
                if !snow_flakes.insert(nm_sflake) {
                    dupe_seen = true;
                }
            }
        }
    }

    println!(
        "{}",
        if dupe_seen {
            "Twin snowflakes found."
        } else {
            "No two snowflakes are alike."
        }
    );
}

/// Format the snowflake accoring to a specific algorithm
fn normalise_snowflake(snowflake: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut new_snow_seq = Vec::new();
    let mut sn_mins = Vec::new();

    /* Find the indexes of the smallest arm on the snowflake. */
    let min_val = snowflake.iter().min().unwrap();
    for arm_idx in 0..NUM_SNOW_ARMS {
        if snowflake[arm_idx] == *min_val {
            sn_mins.push(arm_idx);
        }
    }

    /* Create a linear view of the Snowflake moving from the minimums. */
    for m_idx in sn_mins.into_iter() {
        let lft_arm = snowflake[(m_idx + 1) % NUM_SNOW_ARMS];
        let rgt_arm = snowflake[(m_idx + NUM_SNOW_ARMS - 1) % NUM_SNOW_ARMS];

        /* Pick the smallest number either side of the minimum. */
        if lft_arm < rgt_arm {
            new_snow_seq.push(
                (0..NUM_SNOW_ARMS)
                    .map(|x| snowflake[(m_idx + x) % NUM_SNOW_ARMS])
                    .collect::<Vec<usize>>(),
            );
        } else if lft_arm > rgt_arm {
            new_snow_seq.push(
                (0..NUM_SNOW_ARMS)
                    .map(|x| snowflake[(m_idx + NUM_SNOW_ARMS - x) % NUM_SNOW_ARMS])
                    .collect::<Vec<usize>>(),
            );
        } else {
            new_snow_seq.push(
                (0..NUM_SNOW_ARMS)
                    .map(|x| snowflake[(m_idx + x) % NUM_SNOW_ARMS])
                    .collect::<Vec<usize>>(),
            );
            new_snow_seq.push(
                (0..NUM_SNOW_ARMS)
                    .map(|x| snowflake[(m_idx + NUM_SNOW_ARMS - x) % NUM_SNOW_ARMS])
                    .collect::<Vec<usize>>(),
            )
        }
    }
    return new_snow_seq;
}

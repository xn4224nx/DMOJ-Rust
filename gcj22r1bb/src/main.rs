/*
 * Google Code Jam '22 Round 1B Problem B - Controlled Inflation
 * https://dmoj.ca/problem/gcj22r1bb
 */

use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();

    /* Read the number of cases. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_cases = buffer.trim_end().parse::<usize>().unwrap();

    for case_idx in 0..num_cases {
        buffer.clear();

        /* How many customers are there and how many products do they have each. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let prob_dims = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut pressures = Vec::with_capacity(prob_dims[0]);

        /* Read the pressure values from the customers. */
        for _ in 0..prob_dims[0] {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();

            /* Find the lasrgest and smallest pressure from each. */
            let mut cust_min_pres = u64::MAX;
            let mut cust_max_pres = u64::MIN;

            for raw_pres_val in buffer.trim_end().split_whitespace() {
                if let Ok(pres_val) = raw_pres_val.parse::<u64>() {
                    if pres_val < cust_min_pres {
                        cust_min_pres = pres_val;
                    }
                    if pres_val > cust_max_pres {
                        cust_max_pres = pres_val;
                    }
                }
            }
            pressures.push((cust_min_pres, cust_max_pres))
        }
        println!("Case #{}: {}", case_idx + 1, comparison_solver(&pressures));
    }
}

fn comparison_solver(pressures: &Vec<(u64, u64)>) -> u64 {
    let mut curr_preses = HashMap::from([(0, 0)]);

    for (min_pres, max_pres) in pressures.iter() {
        let mut new_preses = HashMap::new();

        for (path_pres, path_cost) in curr_preses.drain() {
            new_preses.insert(
                *min_pres,
                std::cmp::min(
                    new_preses.get(&min_pres).cloned().unwrap_or(u64::MAX),
                    path_cost + max_pres.abs_diff(path_pres) + max_pres.abs_diff(*min_pres),
                ),
            );

            new_preses.insert(
                *max_pres,
                std::cmp::min(
                    new_preses.get(&max_pres).cloned().unwrap_or(u64::MAX),
                    path_cost + min_pres.abs_diff(path_pres) + min_pres.abs_diff(*max_pres),
                ),
            );
        }
        curr_preses = new_preses;
    }
    return curr_preses.into_values().min().unwrap();
}

/*
 * Google Code Jam '22 Round 1B Problem B - Controlled Inflation
 * https://dmoj.ca/problem/gcj22r1bb
 */

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

        /* Calculate the internal pressure changes */
        let internal_changes = pressures.iter().map(|(x, y)| x.abs_diff(*y)).sum::<u64>();

        /* The pump starts at zero pressure and zero buttons pressed. */
        let mut paths = vec![(0, 0)];

        for cust_idx in 0..prob_dims[0] {
            let mut new_paths = Vec::with_capacity(paths.len() * 2);

            /* Explore the posibility of either path being chosen. */
            for prev_path_idx in 0..paths.len() {
                /* Continue at the min pressure. */
                new_paths.push((
                    paths[prev_path_idx].0 + pressures[cust_idx].0.abs_diff(paths[prev_path_idx].1),
                    pressures[cust_idx].1,
                ));

                /* Continue at the max pressure. */
                new_paths.push((
                    paths[prev_path_idx].0 + pressures[cust_idx].1.abs_diff(paths[prev_path_idx].1),
                    pressures[cust_idx].0,
                ));
            }

            /* Prune some pathways to get a lower the memory footprint. */
            if cust_idx % 5 == 0 {
                new_paths.sort();
                new_paths.resize(new_paths.len() / 2, (0, 0))
            }

            /* Overwrite the old cycles values. */
            paths = new_paths;
        }

        println!(
            "Case #{}: {}",
            case_idx + 1,
            internal_changes + paths.into_iter().map(|x| x.0).min().unwrap()
        );
    }
}

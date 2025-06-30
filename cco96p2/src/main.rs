/*
 * CCO '96 P2 - SafeBreaker
 * https://dmoj.ca/problem/cco96p2
 */

fn main() {
    let all_values = (0..10_000)
        .map(|x| format!("{:0>4}", x).chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut solutions: Vec<bool> = Vec::with_capacity(all_values.len());
    let mut buffer = String::new();

    /* Read the number of test cases. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_test_cases = buffer.trim().parse::<usize>().unwrap();

    /* Read the test cases themselves. */
    for _ in 0..num_test_cases {
        solutions = vec![true; all_values.len()];
        buffer.clear();

        /* How many guesses does the test case have? */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let num_guesses = buffer.trim().parse::<usize>().unwrap();

        /* Read the guesses. */
        for _ in 0..num_guesses {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();

            /* Parse the guess */
            let raw = buffer.trim().chars().collect::<Vec<char>>();
            let correct_digits = raw[5].to_digit(10).unwrap() as usize;
            let misspla_digits = raw[7].to_digit(10).unwrap() as usize;

            /* Eliminate possible answers using the guess. */
            'num_check: for num in 0..all_values.len() {
                let mut digits_in_right_place = vec![false; 4];
                let mut digits_used = vec![false; 4];

                if !solutions[num] {
                    continue 'num_check;
                }

                /* Determine what digits match. */
                for dig_idx in 0..4 {
                    /* Count the digits that appear in the right place. */
                    if raw[dig_idx] == all_values[num][dig_idx] {
                        digits_in_right_place[dig_idx] = true;
                        continue;
                    }

                    /* Count digits that are jumbled up.  */
                    for gue_idx in 0..4 {
                        if raw[gue_idx] == all_values[num][dig_idx] {
                            digits_used[gue_idx] = true;
                        }
                    }
                }

                /* Calculate the correct and mixed up digits */
                let mut num_correct = 0;
                let mut num_used = 0;

                for dig_idx in 0..4 {
                    if digits_in_right_place[dig_idx] {
                        num_correct += 1;
                    } else if digits_used[dig_idx] {
                        num_used += 1;
                    }
                }

                /* Determine if this number matches the guess */
                if num_correct != correct_digits || num_used != misspla_digits {
                    solutions[num] = false;
                }
            }
        }

        /* Determine the case result and print it to stdout. */
        let possible_sols = solutions.iter().filter(|x| **x).count();

        if possible_sols > 1 {
            println!("indeterminate");
        } else if possible_sols == 0 {
            println!("impossible");
        } else {
            for idx in 0..all_values.len() {
                if solutions[idx] {
                    println!("{:0>4}", idx);
                    break;
                }
            }
        };
    }
}

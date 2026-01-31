/*
 * CCC '10 J2 - Up and Down
 * https://dmoj.ca/problem/ccc10j2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the five values of the problem. */
    for _ in 0..5 {
        std::io::stdin().read_line(&mut buffer).unwrap();
    }

    /* Parse the values. */
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* How far does each person get? */
    let n_dist = dist_traveled(data[0], data[1], data[4]);
    let b_dist = dist_traveled(data[2], data[3], data[4]);

    println!(
        "{}",
        if n_dist < b_dist {
            "Byron"
        } else if n_dist > b_dist {
            "Nikky"
        } else {
            "Tied"
        }
    );
}

fn dist_traveled(step_fwd: usize, step_bck: usize, num_steps: usize) -> usize {
    let mut cyc_dist = (step_fwd - step_bck) * (num_steps / (step_fwd + step_bck));
    let cyc_rem = num_steps % (step_fwd + step_bck);

    /* Does the final cycle start going negative? */
    cyc_dist += if cyc_rem <= step_fwd {
        cyc_rem
    } else {
        2 * step_fwd - cyc_rem
    };
    return cyc_dist;
}

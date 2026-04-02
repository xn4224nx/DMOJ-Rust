/*
 * Educational DP Contest AtCoder B - Frog 2
 * https://dmoj.ca/problem/dpb
 */

fn main() {
    let (heights, max_skip) = read_plat_heights();
    println!("{}", min_travel_cost(&heights, max_skip));
}

/// Read the platform sizes and the max skip from STDIN
fn read_plat_heights() -> (Vec<usize>, usize) {
    let mut buffer = String::new();

    /* Extract the problem metadata. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let meta = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Parse the sizes of the platforms. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let plat_heights = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    return (plat_heights, meta[1]);
}

/// Find the minimum traversal cost to reach the end of a list of platforms.
fn min_travel_cost(plat_heights: &Vec<usize>, maxm_skip: usize) -> usize {
    let mut min_cost_plat = vec![usize::MAX; plat_heights.len()];

    /* The first platform is reached for free. */
    min_cost_plat[0] = 0;

    /* Move towards the end working out the minimum cost to reach each platform. */
    for plt_idx in 0..(plat_heights.len() - 1) {
        /* Evaluate the different jumps. */
        for jmp_mag in 1..=maxm_skip {
            if plt_idx + jmp_mag < plat_heights.len() {
                let jmp_cost = min_cost_plat[plt_idx]
                    + plat_heights[plt_idx + jmp_mag].abs_diff(plat_heights[plt_idx]);

                /* Has a new lower lower route to this platform been found? */
                if jmp_cost < min_cost_plat[plt_idx + jmp_mag] {
                    min_cost_plat[plt_idx + jmp_mag] = jmp_cost
                }
            }
        }
    }
    return min_cost_plat[plat_heights.len() - 1];
}

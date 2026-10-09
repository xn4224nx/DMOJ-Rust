/*
 * Triple Fat Ladies
 * https://dmoj.ca/problem/a3
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of test cases. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_cases = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the inital value an calculate the next cube that ends with 888. */
    for _ in 0..num_cases {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let init_val = buffer.trim_end().parse::<u64>().unwrap();

        /* Cubes ending in 888 follow the formula 250x + 192 */
        let cube_idx = init_val.saturating_sub(192) / 250;

        /* Reconstruct the next cube that ends with 888. */
        println!(
            "{}",
            if init_val <= 192 {
                192
            } else {
                250 * (cube_idx + 1) + 192
            }
        );
    }
}

/*
 * DWITE '07 R5 #3 - Parity bit
 * https://dmoj.ca/problem/dwite07c5p3
 */

const NUM_INPUTS: usize = 5;

fn main() {
    let mut buffer = String::new();

    for _ in 0..NUM_INPUTS {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Parse the integer input. */
        let in_val = buffer.trim_end().parse::<u8>().unwrap();

        /* What is the parity bit for this input value? */
        println!("{}", in_val.count_ones() % 2);
    }
}

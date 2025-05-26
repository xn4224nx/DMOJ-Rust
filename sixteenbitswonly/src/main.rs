/*
 * 16 BIT S/W ONLY
 * https://dmoj.ca/problem/16bitswonly
 */

fn main() {
    let mut buffer = String::new();

    /* Read the count of the numbers. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_count = buffer.trim().parse::<usize>().unwrap();

    /* Read the calculations line by line and print a result. */
    for _ in 0..num_count {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let line_vals: Vec<i64> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        if line_vals[0] * line_vals[1] == line_vals[2] {
            println!("POSSIBLE DOUBLE SIGMA");
        } else {
            println!("16 BIT S/W ONLY");
        }
    }
}

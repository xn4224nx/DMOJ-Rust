/*
 * An Animal Contest 3 P1 - Monkey Shopping
 * https://dmoj.ca/problem/aac3p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read and parse the input data from STDIN. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Determine where to go. */
    println!(
        "{}",
        if data[0] < data[1] && data[2] < data[3] {
            "Go to the department store"
        } else if data[0] < data[1] {
            "Go to the grocery store"
        } else if data[2] < data[3] {
            "Go to the pharmacy"
        } else {
            "Stay home"
        }
    );
}

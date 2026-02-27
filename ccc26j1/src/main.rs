/*
 * CCC '26 J1 - Concert Tickets
 * https://dmoj.ca/problem/ccc26j1
 */

const NUM_VARS: usize = 3;

fn main() {
    let mut buffer = String::new();

    /* Read and parse the three variables. */
    for _ in 0..NUM_VARS {
        std::io::stdin().read_line(&mut buffer).unwrap();
    }
    let in_data = buffer
        .trim_end()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    assert_eq!(in_data.len(), NUM_VARS);

    /* can the ticket be purchased?  */
    println!(
        "{}",
        if in_data[0] + in_data[2] > in_data[1] {
            String::from("N")
        } else {
            format!("Y {}", in_data[1] - in_data[2] - in_data[0])
        }
    );
}

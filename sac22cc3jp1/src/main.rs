/*
 * SAC '22 Code Challenge 3 Junior P1 - Normal Problem
 * https://dmoj.ca/problem/sac22cc3jp1
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Detect if Mr. DeMello has been mentioned. */
    println!(
        "{}",
        if buffer.contains("demello") {
            "liar"
        } else {
            "what are we going to do?"
        }
    );
}

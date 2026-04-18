/*
 * SAC '22 Code Challenge 3 Junior P2 - Normal Lines
 * https://dmoj.ca/problem/sac22cc3jp2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the points. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Parse the points. */
    let pnt_data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!(
        "{}",
        if pnt_data[0] == pnt_data[2] {
            "y-axis"
        } else if pnt_data[1] == pnt_data[3] {
            "x-axis"
        } else {
            "neither"
        }
    );
}

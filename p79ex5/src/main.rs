/*
 * BlueBook - Multiple
 * https://dmoj.ca/problem/p79ex5
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Parse the two values. */
    let nums = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Output yes if the first number is a multiple of the second number. */
    println!(
        "{}",
        match nums[0] % nums[1] == 0 {
            true => "yes",
            false => "no",
        }
    );
}

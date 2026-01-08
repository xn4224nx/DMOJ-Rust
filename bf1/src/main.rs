/*
 * List Minimum
 * https://dmoj.ca/problem/bf1
 */

fn main() {
    let mut buffer = String::new();

    /* How long is the list. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let list_len = buffer.trim_end().parse::<usize>().unwrap();

    let mut nums = Vec::with_capacity(list_len);

    /* Read the list elements. */
    for _ in 0..list_len {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        nums.push(buffer.trim_end().parse::<usize>().unwrap());
    }

    /* Print the list smallest to largest. */
    nums.sort();

    for l_idx in 0..list_len {
        println!("{}", nums[l_idx]);
    }
}

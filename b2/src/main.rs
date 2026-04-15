/*
 * Adding Reversed Numbers
 * https://dmoj.ca/problem/b2
 */

fn main() {
    let mut buffer = String::new();

    /* How numbers will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let nums_cnt = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the numbers. */
    for _ in 0..nums_cnt {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Parse the two values to sum. */
        let data = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Calculate the reversed sum of the two numbers. */
        println!(
            "{}",
            reverse_number(reverse_number(data[0]) + reverse_number(data[1]))
        );
    }
}

/// Reverse the digits of a number.
fn reverse_number(num: usize) -> usize {
    let base = 10;
    let mut orig = num;
    let mut new = 0;

    while orig != 0 {
        new = new * base + orig % base;
        orig /= base;
    }

    return new;
}

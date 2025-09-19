/*
 * DMOPC '16 Contest 4 P1 - Fast Exponents
 * https://dmoj.ca/problem/dmopc16c4p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the total numbers that need testing. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let total_nums = buffer.trim_end().parse::<usize>().unwrap();

    /* Read sone numbers and determine if they are powers of two. */
    for _ in 0..total_nums {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        println!(
            "{}",
            match buffer.trim_end().parse::<u64>().unwrap().is_power_of_two() {
                true => 'T',
                false => 'F',
            }
        );
    }
}

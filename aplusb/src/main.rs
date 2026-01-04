/*
 * A Plus B
 * https://dmoj.ca/problem/aplusb
 */

fn main() {
    let mut buffer = String::new();

    /* How many sums are there going to be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_sums = buffer.trim_end().parse::<usize>().unwrap();

    /* Do the sums and print the result. */
    for _ in 0..num_sums {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let vals = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        println!("{}", vals[0] + vals[1]);
    }
}

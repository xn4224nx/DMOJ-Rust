/*
 * CCC '19 J2 - Time to Decompress
 * https://dmoj.ca/problem/ccc19j2
 */

fn main() {
    let mut buffer = String::new();

    /* How many messages are there. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_msgs = buffer.trim_end().parse::<usize>().unwrap();

    /* Read and decode the messages. */
    for _ in 0..num_msgs {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let raw_data: Vec<&str> = buffer.trim_end().split_whitespace().collect();
        let reps = raw_data[0].parse::<usize>().unwrap();

        /* Print the decoded message. */
        for _ in 0..reps {
            print!("{}", raw_data[1]);
        }
        println!();
    }
}

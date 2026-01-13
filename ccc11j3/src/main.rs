/*
 * CCC '11 J3 - Sumac Sequences
 * https://dmoj.ca/problem/ccc11j3
 */

fn main() {
    let mut buffer = String::new();
    let mut seq_len = 0;

    /* Read the first two numbers of the sequence. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut t1 = buffer.trim_end().parse::<usize>().unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut t2 = buffer.trim_end().parse::<usize>().unwrap();

    /* Determine the sequence length. */
    while t2 <= t1 {
        let temp = t2;
        t2 = t1.saturating_sub(t2);
        t1 = temp;
        seq_len += 1;
    }

    println!("{}", seq_len + 2);
}

/*
 * COCI '09 Contest 1 #2 Domino
 * https://dmoj.ca/problem/coci09c1p2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the size of the set. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let set_size = buffer.trim_end().parse::<usize>().unwrap() + 1;

    /* Determine the total number of dots in the totality of pair combinations. */
    let total = (1..set_size).map(|x| x * (set_size + 1)).sum::<usize>();

    println!("{}", total);
}

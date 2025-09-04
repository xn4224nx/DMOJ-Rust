/*
 * CCC '14 J2 - Vote Count
 * https://dmoj.ca/problem/ccc14j2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the votes */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_votes = buffer.trim_end().parse::<usize>().unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Count the votes for A. */
    let num_a: usize = buffer.chars().filter(|x| *x == 'A').count();

    if num_a * 2 == num_votes {
        println!("Tie");
    } else if num_a * 2 > num_votes {
        println!("A");
    } else {
        println!("B");
    }
}

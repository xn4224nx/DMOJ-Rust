/*
 * An Animal Contest 2 P1 - Koala Konundrum
 * https://dmoj.ca/problem/aac2p1
 */

fn main() {
    let mut buffer = String::new();
    let mut letter_cnts = vec![0; 26];

    /* Read the letters */
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Count the occurances of each letter in the string. */
    for letter in buffer.trim_end().chars() {
        letter_cnts[letter as usize - 'a' as usize] += 1;
    }

    /* Determine the number of odd-letters. */
    let num_odd = letter_cnts.into_iter().filter(|x| x % 2 != 0).count();

    /* What is the minimum score. */
    println!("{}", std::cmp::max(1, num_odd));
}

/*
 * ECOO '21 P2 - DNA Derren
 * https://dmoj.ca/problem/ecoo21p2
 */

fn main() {
    let mut buffer = String::new();
    let mut final_product = String::new();
    let mut prev_letter: Option<char> = None;

    /* Read the initial DNA Sequence. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Insert a space if both letters are A or neither are. */
    for curr_letter in buffer.chars() {
        if (curr_letter == 'A' && prev_letter == Some('A'))
            || (curr_letter != 'A' && prev_letter != Some('A'))
        {
            final_product.push(' ');
        }

        final_product.push(curr_letter);
        prev_letter = Some(curr_letter);
    }
    print!("{}", final_product);
}

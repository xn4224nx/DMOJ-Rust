/*
 * CCC '13 J2 - Rotating letters
 * https://dmoj.ca/problem/ccc13j2
 */

fn main() {
    let rev_letters = vec!['I', 'O', 'S', 'H', 'Z', 'X', 'N'];
    let mut buffer = String::new();

    /* Read the word and remove the newline char at the end. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.pop();

    /* Detect banned letters. */
    for letter in buffer.chars() {
        if !rev_letters.contains(&letter) {
            println!("NO");
            return;
        }
    }

    println!("YES");
}

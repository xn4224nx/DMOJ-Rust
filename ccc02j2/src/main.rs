/*
 * CCC '02 J2 - AmeriCanadian
 * https://dmoj.ca/problem/ccc02j2
 */

fn main() {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    let exit_cmd = vec!['q', 'u', 'i', 't', '!'];
    let mut buffer = String::new();

    loop {
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Convert to a vector of chars and remove whitespace. */
        let mut word: Vec<char> = buffer.trim().chars().collect();

        /* Detect the quit command. */
        if word == exit_cmd {
            break;
        }

        /* Detect American specific words and correct them. */
        if word.len() > 4
            && !vowels.contains(&word[word.len() - 3])
            && word[word.len() - 2] == 'o'
            && word[word.len() - 1] == 'r'
        {
            word.pop();
            word.push('u');
            word.push('r');
        }
        println!("{}", word.into_iter().collect::<String>());
        buffer.clear();
    }
}

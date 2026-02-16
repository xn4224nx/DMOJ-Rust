/*
 * CCC '06 J3 - Cell-Phone Messaging
 * https://dmoj.ca/problem/ccc06j3
 */

fn main() {
    let mut buffer = String::new();

    loop {
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Has the exit command been given? */
        if buffer == "halt\n" {
            break;
        }

        /* Remove the new line character. */
        buffer.pop();

        println!("{}", word_time(&buffer.bytes().collect()));
        buffer.clear();
    }
}

fn word_time(word: &Vec<u8>) -> usize {
    let mut time = 0;
    let letter_presses = vec![
        1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 4, 1, 2, 3, 1, 2, 3, 4,
    ];
    let letter_buttons = vec![
        0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7,
    ];

    /* How many button presses would each letter need. */
    for ltr_idx in 0..word.len() {
        time += letter_presses[(word[ltr_idx] - b'a') as usize];

        /* Does a pause need to be inserted. */
        if ltr_idx < word.len() - 1
            && letter_buttons[(word[ltr_idx] - b'a') as usize]
                == letter_buttons[(word[ltr_idx + 1] - b'a') as usize]
        {
            time += 2;
        }
    }
    return time;
}

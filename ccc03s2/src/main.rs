/*
 * CCC '03 S2 - Poetry
 * https://dmoj.ca/problem/ccc03s2
 */

const LINES_PER_VERSE: usize = 4;

fn main() {
    let mut buffer = String::new();

    /* How many verses are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_verses = buffer.trim_end().parse::<usize>().unwrap();

    for _ in 0..num_verses {
        let mut last_syllables = Vec::new();

        /* For each line get its final sylable from its final word. */
        for _ in 0..LINES_PER_VERSE {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();
            last_syllables.push(extract_syllable(
                buffer.trim_end().split_whitespace().last().unwrap(),
            ));
        }

        /* Determine what the rhyming scheme is. */
        println!(
            "{}",
            if last_syllables[0] == last_syllables[1]
                && last_syllables[2] == last_syllables[3]
                && last_syllables[0] == last_syllables[3]
            {
                "perfect"
            } else if last_syllables[0] == last_syllables[1]
                && last_syllables[2] == last_syllables[3]
            {
                "even"
            } else if last_syllables[0] == last_syllables[2]
                && last_syllables[1] == last_syllables[3]
            {
                "cross"
            } else if last_syllables[0] == last_syllables[3]
                && last_syllables[1] == last_syllables[2]
            {
                "shell"
            } else {
                "free"
            }
        );
    }
}

fn extract_syllable(word: &str) -> String {
    let word_chars = word.to_lowercase().chars().collect::<Vec<char>>();
    let mut last_vowel_idx = usize::MAX;

    /* Find the position of the last vowel in the word. */
    for char_idx in 0..word_chars.len() {
        if vec!['a', 'e', 'i', 'o', 'u'].contains(&word_chars[char_idx]) {
            last_vowel_idx = char_idx;
        }
    }

    /*  If a word has no vowel, then the last syllable is the whole word. */
    return if last_vowel_idx == usize::MAX {
        word.to_string()

    /* Extract the final syllable. */
    } else {
        (last_vowel_idx..word_chars.len())
            .map(|x| word_chars[x])
            .collect()
    };
}

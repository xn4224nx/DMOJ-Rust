/*
 * CCC '15 J3 - Rövarspråket
 * https://dmoj.ca/problem/ccc15j3
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    print!("{}", robbers_lang(buffer));
}

pub fn robbers_lang(orig: String) -> String {
    let mut convert = String::new();

    for letter in orig.chars() {
        convert.push(letter);

        if is_consonant(letter) {
            convert.push(closest_vowel(letter));
            convert.push(next_conts(letter));
        }
    }
    return convert;
}

/// Determine if a letter is a consonant
pub fn is_consonant(letter: char) -> bool {
    match letter {
        'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' | 's'
        | 't' | 'v' | 'w' | 'x' | 'y' | 'z' => true,
        _ => false,
    }
}

/// Find the closet vowel to the letter.
pub fn closest_vowel(letter: char) -> char {
    match letter {
        'b' | 'c' => 'a',
        'd' | 'f' | 'g' => 'e',
        'h' | 'j' | 'k' | 'l' => 'i',
        'm' | 'n' | 'p' | 'q' | 'r' => 'o',
        's' | 't' | 'v' | 'w' | 'x' | 'y' | 'z' => 'u',
        _ => panic!("Unsupported letter '{}'", letter),
    }
}

pub fn next_conts(letter: char) -> char {
    match letter {
        'b' => 'c',
        'c' => 'd',
        'd' => 'f',
        'f' => 'g',
        'g' => 'h',
        'h' => 'j',
        'j' => 'k',
        'k' => 'l',
        'l' => 'm',
        'm' => 'n',
        'n' => 'p',
        'p' => 'q',
        'q' => 'r',
        'r' => 's',
        's' => 't',
        't' => 'v',
        'v' => 'w',
        'w' => 'x',
        'x' => 'y',
        'y' => 'z',
        'z' => 'z',
        _ => panic!("Unsupported letter '{}'", letter),
    }
}

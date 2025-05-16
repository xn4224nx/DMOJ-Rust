/*
 * WC '17 Contest 3 J3 - Uncrackable
 * https://dmoj.ca/problem/wc17c3j3
 */

fn main() {
    let mut password = String::new();
    std::io::stdin()
        .read_line(&mut password)
        .expect("Line could not be read!");

    if pass_check(&password) {
        println!("Valid");
    } else {
        println!("Invalid");
    }
}

/// Is the password valid
fn pass_check(secret: &String) -> bool {
    let mut lower_cnt = 0;
    let mut upper_cnt = 0;
    let mut digit_cnt = 0;

    /* Is the length correct? */
    if secret.len() < 9 || secret.len() > 13 {
        return false;
    }

    /* Check the composition of the password */
    for (p_idx, p_char) in secret.chars().enumerate() {
        /* Ignore the last character as it will be a new line */
        if p_idx == secret.len() - 1 {
            break;
        }

        if p_char.is_ascii_digit() {
            digit_cnt += 1;
        } else if p_char.is_ascii_lowercase() {
            lower_cnt += 1;
        } else if p_char.is_ascii_uppercase() {
            upper_cnt += 1;

        /* The password only allows the above three types of char */
        } else {
            return false;
        }
    }

    /* Ensure the password has the right composition. */
    if lower_cnt < 3 || upper_cnt < 2 || digit_cnt < 1 {
        return false;
    }
    return true;
}

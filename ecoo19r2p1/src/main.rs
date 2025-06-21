/*
 * ECOO '19 R2 P1 - Email
 * https://dmoj.ca/problem/ecoo19r2p1
 */

use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();

    for _ in 0..10 {
        let mut emails: HashSet<String> = HashSet::new();

        /* How many emails will follow? */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let num_emails = buffer.trim().parse::<usize>().unwrap();

        /* Read the emails and strip them down before recording them. */
        for _ in 0..num_emails {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();
            emails.insert(strip_email(&buffer));
        }
        println!("{}", emails.len());
    }
}

pub fn strip_email(messy_email: &String) -> String {
    let mut passed_at = false;
    let mut plus_encountered = false;
    let mut cleaned_email = String::new();

    /* Iterate over the characters one by one */
    for em_char in messy_email.chars() {
        let new_char = em_char.to_ascii_lowercase();

        /* Some chars before the @ need to be ignored. */
        if !passed_at {
            if em_char == '.' {
                continue;
            }
            if em_char == '+' {
                plus_encountered = true;
                continue;
            }
        }

        /* Detect the split in the email*/
        if new_char == '@' {
            passed_at = true;
            plus_encountered = false;
        }

        /* Save the character */
        if !plus_encountered {
            cleaned_email.push(new_char);
        }
    }
    return cleaned_email;
}

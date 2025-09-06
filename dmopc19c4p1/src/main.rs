/*
 * DMOPC '19 Contest 4 P1 - Valid Strings
 * https://dmoj.ca/problem/dmopc19c4p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of strings to assess. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_str = buffer
        .trim()
        .parse::<usize>()
        .expect("Number of strings could not be parsed!");

    /* Read the strings and print if they are valid or not. */
    for _ in 0..num_str {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        println!("{}", is_string_valid(&buffer));
    }
}

/// Determine if the supplied string is valid and return either YES or NO
/// indicating the result.
fn is_string_valid(test_data: &String) -> String {
    let mut bracket_depth: usize = 0;
    let mut bracket_balance: i32 = 0;

    /* Ensure '(' and ')' come in valid pairs. */
    for s_char in test_data.chars() {
        if s_char == '(' {
            bracket_depth += 1;
            bracket_balance += 1;
        } else if s_char == ')' {
            bracket_depth = bracket_depth.saturating_sub(1);
            bracket_balance -= 1;
        }
    }

    return if bracket_depth != 0 || bracket_balance != 0 {
        String::from("NO")
    } else {
        String::from("YES")
    };
}

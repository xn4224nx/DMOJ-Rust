/*
 * CCC '05 S1 - Snow Calls
 * https://dmoj.ca/problem/ccc05s1
 */

fn main() {
    let mut buffer = String::new();

    /* How many test cases are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_test_cases = buffer.trim_end().parse::<usize>().unwrap();

    /* Process the test cases. */
    for _ in 0..num_test_cases {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        println!("{}", clean_raw_phone_num(&buffer));
    }
}

fn clean_raw_phone_num(raw_num: &str) -> String {
    let mut cln_num = String::new();

    /* Convert the characters into digits */
    for val in raw_num.chars() {
        if val.is_digit(10) {
            cln_num.push(val);
        } else if val.is_ascii_uppercase() {
            cln_num.push(match val {
                'A' | 'B' | 'C' => '2',
                'D' | 'E' | 'F' => '3',
                'G' | 'H' | 'I' => '4',
                'J' | 'K' | 'L' => '5',
                'M' | 'N' | 'O' => '6',
                'P' | 'Q' | 'R' | 'S' => '7',
                'T' | 'U' | 'V' => '8',
                'W' | 'X' | 'Y' | 'Z' => '9',
                _ => panic!("Unknown letter!"),
            });
        }

        /* Add in the hyphen dividers. */
        if cln_num.len() == 3 || cln_num.len() == 7 {
            cln_num.push('-');
        }

        /* Ignore data past the 10th digit */
        if cln_num.len() > 11 {
            break;
        }
    }
    return cln_num;
}

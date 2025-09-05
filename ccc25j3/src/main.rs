/*
 * CCC '25 J3 - Product Codes
 * https://dmoj.ca/problem/ccc25j3
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of product codes.*/
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_codes = buffer.trim().parse::<usize>().unwrap();

    /* Read the product codes. */
    for _ in 0..num_codes {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();

        let mut new_code = String::new();
        let mut temp_num = String::new();
        let mut code_sum: i32 = 0;

        for code in buffer.chars() {
            /* Preserve uppercase letters. */
            if code.is_uppercase() {
                new_code.push(code);
            }

            /* Save parts of numbers. */
            if code.is_ascii_digit() || (code == '-' && temp_num.is_empty()) {
                temp_num.push(code);

            /* Detect the end of a number. */
            } else if !temp_num.is_empty() {
                code_sum += temp_num.parse::<i32>().unwrap();
                temp_num.clear();

                /* Deal with two negative numbers together. */
                if code == '-' {
                    temp_num.push(code);
                }
            }
        }
        buffer.clear();

        /* Ensure that there is no number left over. */
        if !temp_num.is_empty() {
            code_sum += temp_num.parse::<i32>().unwrap();
            temp_num.clear();
        }

        /* Print the transformed code. */
        println!("{}{}", new_code, code_sum);
    }
}

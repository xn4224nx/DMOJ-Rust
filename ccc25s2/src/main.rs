/*
 * CCC '25 S2 - Cryptogram Cracking Club
 * https://dmoj.ca/problem/ccc25s2
 */

fn main() {
    let mut buffer = String::new();
    let mut comp_char = Vec::new();
    let mut comp_count = Vec::new();

    /* Read and parse the compressed pattern. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut temp_num = String::new();
    for raw_chr in buffer.chars() {
        if raw_chr.is_alphabetic() {
            comp_char.push(raw_chr);

            /* Parse the previous number */
            if !temp_num.is_empty() {
                comp_count.push(temp_num.parse::<usize>().unwrap());
                temp_num.clear();
            }
        } else if raw_chr.is_digit(10) {
            temp_num.push(raw_chr);
        }
    }

    /* Catch the last number. */
    comp_count.push(temp_num.parse::<usize>().unwrap());

    /* Read the index of the character we want. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let get_char_idx = buffer.trim_end().parse::<usize>().unwrap();

    /* Determine how far into pattern the desired character is. */
    let desired_idx = get_char_idx % comp_count.iter().sum::<usize>();

    /* Determine what char in the pattern should be shown. */
    let mut culm_sum = 0;

    for seg_idx in 0..comp_char.len() {
        culm_sum += comp_count[seg_idx];
        if desired_idx < culm_sum {
            println!("{}", comp_char[seg_idx]);
            break;
        }
    }
}

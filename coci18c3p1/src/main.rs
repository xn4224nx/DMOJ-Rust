/*
 * COCI '18 Contest 3 #1 Magnus
 * https://dmoj.ca/problem/coci18c3p1
 */

fn main() {
    let blck = vec!['H', 'O', 'N', 'I'];
    let mut blk_count = 0;
    let mut blk_index = 0;

    /* Read the stream of letters */
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    /* Iterate over the stream and count the number of possible HONI blocks. */
    for s_char in input.chars() {
        /* Check for the next char in the block appearing */
        if s_char == blck[blk_index] {
            blk_index += 1;

            /* Check for the end of the block. */
            if blk_index == blck.len() {
                blk_count += 1;
                blk_index = 0;
            }
        }
    }
    println!("{}", blk_count);
}

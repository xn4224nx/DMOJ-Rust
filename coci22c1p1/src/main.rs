/*
 * COCI '22 Contest 1 #1 Desni Klik
 * https://dmoj.ca/problem/coci22c1p1
 */

const NUM_IND: char = '#';

fn main() {
    let mut buffer = String::new();

    /* Read the details of the problem. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Set the key variables for the problem. */
    let (num_nfp, num_rows, num_cols) = (data[0], data[1], data[2]);

    /* Read and determine the inscurity of each NFP in turn.  */
    for _ in 0..num_nfp {
        let mut timeframe_secur = vec![0; num_cols];

        /* Read the NFP. */
        for row_idx in 0..num_rows {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();

            for (col_idx, col_val) in buffer.chars().enumerate() {
                if col_val == NUM_IND && col_idx < timeframe_secur.len() {
                    timeframe_secur[col_idx] = num_rows - row_idx;
                }
            }
        }

        /* Calculate the NFPs insecurity. */
        let mut min_security = usize::MAX;
        let mut max_security = usize::MIN;

        for sec_val in timeframe_secur.into_iter() {
            if sec_val < min_security {
                min_security = sec_val;
            }
            if sec_val > max_security {
                max_security = sec_val;
            }
        }
        println!("{}", max_security.abs_diff(min_security));
    }
}

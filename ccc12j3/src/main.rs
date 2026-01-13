/*
 * CCC '12 J3 - Icon Scaling
 * https://dmoj.ca/problem/ccc12j3
 */

fn main() {
    let mut buffer = String::new();
    let mut scaled_icon = String::new();
    let icon = vec![
        vec!['*', 'x', '*'],
        vec![' ', 'x', 'x'],
        vec!['*', ' ', '*'],
    ];

    /* Read the scalling factor. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let scl_fac = buffer.trim_end().parse::<usize>().unwrap();

    /* Construct the scaled icon */
    for orig_row_idx in 0..icon.len() {
        for _ in 0..scl_fac {
            for orig_col_idx in 0..icon[orig_row_idx].len() {
                for _ in 0..scl_fac {
                    scaled_icon.push(icon[orig_row_idx][orig_col_idx]);
                }
            }
            scaled_icon.push('\n');
        }
    }
    print!("{}", scaled_icon);
}

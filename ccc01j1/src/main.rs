/*
 * CCC '01 J1 - Dressing Up
 * https://dmoj.ca/problem/ccc01j1
 */

const FULL_PIXEL: char = '*';
const EMPT_PIXEL: char = ' ';

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    print!("{}", tie_bow(buffer.trim_end().parse::<usize>().unwrap()));
}

fn tie_bow(b_size: usize) -> String {
    let mut bowtie = String::new();

    /* The size of a bow-tie must be odd. */
    assert_ne!(b_size % 2, 0);

    /* Top half of the bow. */
    for row_idx in 0..=(b_size / 2) {
        let reach = 2 * row_idx + 1;

        for col_idx in 0..(2 * b_size) {
            bowtie.push(if col_idx >= reach && col_idx < (2 * b_size) - reach {
                EMPT_PIXEL
            } else {
                FULL_PIXEL
            });
        }
        bowtie.push('\n');
    }

    /* Bottom half of the bow. */
    for row_idx in (0..(b_size / 2)).rev() {
        let reach = 2 * row_idx + 1;

        for col_idx in 0..(2 * b_size) {
            bowtie.push(if col_idx >= reach && col_idx < (2 * b_size) - reach {
                EMPT_PIXEL
            } else {
                FULL_PIXEL
            });
        }
        bowtie.push('\n');
    }
    return bowtie;
}

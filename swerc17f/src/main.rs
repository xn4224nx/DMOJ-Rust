/*
 * ICPC SWERC 2017 F - Shattered Cake
 * https://dmoj.ca/problem/swerc17f
 */

fn main() {
    println!("{}", find_rectangle_height());
}

/// From STDIN read the width of the rectangle, the widths and heights of the
/// quadralateral pieces. Then calculate the height of the original rectangle.
fn find_rectangle_height() -> usize {
    let mut buffer = String::new();
    let mut quadrangle_area = 0;

    /* What is the width of the original rectangle? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let width = buffer.trim_end().parse::<usize>().unwrap();

    /* How many pieces are there? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_pieces = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the dimensions of the pieces. */
    for _ in 0..num_pieces {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let dims = buffer
            .trim_end()
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        /* Only process valid lines. */
        if dims.len() >= 2 {
            quadrangle_area += dims[0] * dims[1];
        }
    }
    return quadrangle_area / width;
}

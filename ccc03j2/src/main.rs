/*
 * CCC '03 J2 - Picture Perfect
 * https://dmoj.ca/problem/ccc03j2
 */

fn main() {
    let mut buffer = String::new();

    /* Read picture sizes until the exit command is given. */
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let rect_area = buffer.trim_end().parse::<usize>().unwrap();

        /* Has the exist command been given? */
        if rect_area == 0 {
            break;
        }
        min_perimeter(rect_area);
    }
}

/// Determine the minimum area that a reactangle with this area can have.
fn min_perimeter(area: usize) {
    for side_len in (1..area.isqrt() + 1).rev() {
        if area % side_len == 0 {
            let o_sd_len = area / side_len;

            /* Create a human readable summary. */
            println!(
                "Minimum perimeter is {} with dimensions {} x {}",
                2 * o_sd_len + 2 * side_len,
                std::cmp::min(o_sd_len, side_len),
                std::cmp::max(o_sd_len, side_len),
            );
            return;
        }
    }
}

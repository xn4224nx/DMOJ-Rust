/*
 * VM7WC '16 #2 Bronze - G
 * https://dmoj.ca/problem/vmss7wc16c2p1
 */

fn main() {
    let mut buffer = String::new();

    /* What is the size variable of the G? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let g_size = buffer.trim_end().parse::<usize>().unwrap();

    /* Print the big G. */
    print!("{}", letter_g_printer(g_size));
}

fn letter_g_printer(size: usize) -> String {
    let dimension_ln = 5;
    let full_pxl = 'G';
    let empt_pxl = '.';
    let mut out = String::new();

    /* Add in the top bar of the G. */
    for _ in 0..size {
        for _ in 0..(size * dimension_ln) {
            out.push(full_pxl);
        }
        out.push('\n');
    }

    /* Add in the top third of the stem of the G. */
    for _ in 0..size {
        for _ in 0..size {
            out.push(full_pxl);
        }
        for _ in 0..(size * (dimension_ln - 1)) {
            out.push(empt_pxl);
        }
        out.push('\n');
    }

    /* Add in the middle part of the stem of the G. */
    for _ in 0..size {
        for _ in 0..size {
            out.push(full_pxl);
        }

        for _ in 0..(size * (dimension_ln - 3)) {
            out.push(empt_pxl);
        }

        for _ in 0..(size * 2) {
            out.push(full_pxl);
        }
        out.push('\n');
    }

    /* Add in the bottom third of the stem of the G. */
    for _ in 0..size {
        for _ in 0..size {
            out.push(full_pxl);
        }

        for _ in 0..(size * (dimension_ln - 2)) {
            out.push(empt_pxl);
        }

        for _ in 0..size {
            out.push(full_pxl);
        }
        out.push('\n');
    }

    /* Add in the bottom bar of the G. */
    for _ in 0..size {
        for _ in 0..(size * dimension_ln) {
            out.push(full_pxl);
        }
        out.push('\n');
    }

    return out;
}

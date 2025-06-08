/*
 * COCI '19 Contest 5 #1 Emacs
 * https://dmoj.ca/problem/coci19c5p1
 */

use std::collections::HashSet;

fn main() {
    let mut num_rects = 0;
    let mut buffer = String::new();

    /* Read the number of rows and columns. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let dims: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    /* Read the image data. */
    let mut img: HashSet<(usize, usize)> = HashSet::new();
    for row_idx in 0..dims[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        for (col_idx, img_chr) in buffer.chars().enumerate() {
            if img_chr == '*' {
                img.insert((row_idx, col_idx));
            }
        }
    }

    /* Count the number of rectangles in the image. */
    while img.len() > 0 {
        let mut tmp_rect_pnts: HashSet<(usize, usize)> = HashSet::new();

        /* Start with a random point. */
        let mut curr_points: HashSet<(usize, usize)> =
            HashSet::from([img.iter().next().unwrap().clone()]);

        /* Find all the points in this particular rectangle. */
        while curr_points.len() > 0 {
            let mut nxt_pnts: HashSet<(usize, usize)> = HashSet::new();

            /* For this wave of points find other connected points. */
            for c_pnt in curr_points.drain() {
                /* Generate the points possibly adjacent to this one. */
                for n_pnt in vec![
                    (c_pnt.0.overflowing_sub(1).0, c_pnt.1),
                    (c_pnt.0 + 1, c_pnt.1),
                    (c_pnt.0, c_pnt.1.overflowing_sub(1).0),
                    (c_pnt.0, c_pnt.1 + 1),
                ]
                .into_iter()
                {
                    /* Check it is part of the rectangle. */
                    if img.contains(&n_pnt) && !tmp_rect_pnts.contains(&n_pnt) {
                        nxt_pnts.insert(n_pnt);
                    }
                }

                /* Don't check this point again, ever. */
                img.remove(&c_pnt);

                /* Keep a record of all points in this rectangle */
                tmp_rect_pnts.insert(c_pnt);
            }

            /* Prepare for the next wave of points in the rectangle. */
            curr_points = nxt_pnts;
        }
        num_rects += 1;
    }
    println!("{}", num_rects);
}

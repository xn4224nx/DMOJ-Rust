/*
 * COCI '06 Contest 3 #3 Trojke
 * https://dmoj.ca/problem/coci06c3p3
 */

fn main() {
    let mut buffer = String::new();

    /* How big are the sides of the letter grid. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let grid_dim = buffer.trim_end().parse::<usize>().unwrap();

    let mut letter_coords: Vec<(i32, i32)> = Vec::new();

    /* Read the grid and save the coordinates of the letters. */
    for y_idx in 0..grid_dim {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        for (x_idx, sqr_cont) in buffer.chars().enumerate() {
            if sqr_cont.is_uppercase() {
                letter_coords.push((x_idx as i32, y_idx as i32));
            }
        }
    }

    /* Determine if any combination of three letters is in a line. */
    let mut num_triplets = 0;

    /* Iterate over every combination of coordinates. */
    for l0_idx in 0..letter_coords.len() - 2 {
        for l1_idx in l0_idx + 1..letter_coords.len() - 1 {
            for l2_idx in l1_idx + 1..letter_coords.len() {
                /* Points are on a line if they are co-linear */
                if (letter_coords[l2_idx].0 - letter_coords[l0_idx].0)
                    * (letter_coords[l1_idx].1 - letter_coords[l0_idx].1)
                    == (letter_coords[l1_idx].0 - letter_coords[l0_idx].0)
                        * (letter_coords[l2_idx].1 - letter_coords[l0_idx].1)
                {
                    num_triplets += 1;
                }
            }
        }
    }

    println!("{}", num_triplets);
}

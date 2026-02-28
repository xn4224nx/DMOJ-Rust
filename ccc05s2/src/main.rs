/*
 * CCC '05 S2 - Mouse Move
 * https://dmoj.ca/problem/ccc05s2
 */

fn main() {
    let mut buffer = String::new();
    let mut curr_pos = vec![0, 0];

    /* What is the screen size. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let screen_sz = buffer
        .trim_end()
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    /* Respond to the relative moves of the mouse.  */
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Parse the coordinates */
        let mouse_mv = buffer
            .trim_end()
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        /* Has the exit command been given? */
        if mouse_mv[0] == 0 && mouse_mv[1] == 0 {
            break;
        }

        /* Move the mouse. */
        curr_pos[0] += mouse_mv[0];
        curr_pos[1] += mouse_mv[1];

        /* Clamp it to the screen bounds. */
        for dim_i in 0..2 {
            if curr_pos[dim_i] < 0 {
                curr_pos[dim_i] = 0;
            } else if curr_pos[dim_i] > screen_sz[dim_i] {
                curr_pos[dim_i] = screen_sz[dim_i];
            }
        }

        /* Print the new position of the mouse. */
        println!("{} {}", curr_pos[0], curr_pos[1]);
    }
}

/*
 * COCI '09 Contest 6 #1 Kajak
 * https://dmoj.ca/problem/coci09c6p1
 */

const NUM_KAYAKS: usize = 9;

fn main() {
    let mut buffer = String::new();

    /* Read the dimensions of the race. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let race_dims = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut kajak_distance = vec![0; NUM_KAYAKS];
    let mut kajak_position = vec![0; NUM_KAYAKS];

    /* Read how far each kajak has come. */
    for _ in 0..race_dims[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Find the position and number of the kajak in the race. */
        for (rc_idx, rc_img) in buffer.chars().enumerate() {
            if let Some(kajak_name) = rc_img.to_digit(10) {
                kajak_distance[(kajak_name - 1) as usize] = rc_idx;
                break;
            }
        }
    }

    /* Determine the order of each Kajak */
    let mut kajak_used = 0;
    let mut curr_pos = 0;

    while kajak_used < NUM_KAYAKS {
        /* Find the distance of the kajak furthest in the lead. */
        let lead_val = *kajak_distance.iter().max().unwrap();

        /* Set the positions of these kajak's. */
        for kajak_idx in 0..NUM_KAYAKS {
            if kajak_distance[kajak_idx] == lead_val {
                kajak_position[kajak_idx] = curr_pos + 1;
                kajak_distance[kajak_idx] = usize::MIN;
                kajak_used += 1;
            }
        }
        curr_pos += 1;
    }

    /* Print the position of each Kajak. */
    for pos in kajak_position.into_iter() {
        println!("{}", pos);
    }
}

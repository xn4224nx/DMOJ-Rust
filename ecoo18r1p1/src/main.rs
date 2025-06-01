/*
 * ECOO '18 R1 P1 - Willow's Wild Ride
 * https://dmoj.ca/problem/ecoo18r1p1
 */

fn main() {
    let mut buffer = String::new();

    /* Process the datsets */
    for _ in 0..10 {
        buffer.clear();

        /* Read the number of days. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let data: Vec<usize> = buffer
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        /* Sum the time taken for the cat to play with the boxes.  */
        let mut box_time_required = 0;

        /* Read the boxes per day */
        for _ in 0..data[1] {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();

            /* Detect the addition of a new box. */
            if buffer == "B\n" {
                box_time_required += data[0];
            }

            /* The cat plays with a box for a day */
            box_time_required = box_time_required.saturating_sub(1);
        }
        println!("{}", box_time_required);
    }
}

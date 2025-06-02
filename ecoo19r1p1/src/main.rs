/*
 * ECOO '19 R1 P1 - Free Shirts
 * https://dmoj.ca/problem/ecoo19r1p1
 */

fn main() {
    let mut buffer = String::new();

    for _ in 0..10 {
        let mut num_of_washes = 0;

        /* Read the number of clean shirts, the number of events, the number of days. */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let data: Vec<usize> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        /* Read the dates of the conventions. */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let conv_dates: Vec<usize> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let mut total_shirts = data[0];
        let mut clean_shirts = data[0];

        /* Simulate the laundry process, day by day.  */
        for day in 1..=data[2] {
            /* They do the laundry. */
            if clean_shirts <= 0 {
                clean_shirts = total_shirts;
                num_of_washes += 1;
            }

            /* They put on a shirt for the day */
            clean_shirts -= 1;

            /* Check if they attended any events that day. */
            let new_shirts = conv_dates.iter().filter(|x| **x == day).count();

            /* Update the number of shirts. */
            total_shirts += new_shirts;
            clean_shirts += new_shirts;
        }
        println!("{}", num_of_washes);
    }
}

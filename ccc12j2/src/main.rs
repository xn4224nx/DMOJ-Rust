/*
 * CCC '12 J2 - Sounds fishy!
 * https://dmoj.ca/problem/ccc12j2
 */

fn main() {
    let num_readings = 4;
    let mut buffer = String::new();
    let mut readings: Vec<usize> = Vec::with_capacity(num_readings);

    /* Read the depth readings. */
    for _ in 0..num_readings {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        readings.push(buffer.trim().parse::<usize>().unwrap());
    }

    /* Keep a record of the change in heights */
    let mut descnd = false;
    let mut ascend = false;
    let mut equal = false;

    /* Iterate over the readings and identify the overall type of fish. */
    for rd_idx in 1..num_readings {
        if readings[rd_idx - 1] == readings[rd_idx] {
            equal = true;
        } else if readings[rd_idx - 1] < readings[rd_idx] {
            ascend = true;
        } else {
            descnd = true;
        }

        /* Ensure that only one path is being followed. */
        if descnd as u8 + ascend as u8 + equal as u8 > 1 {
            println!("No Fish");
            return;
        }
    }

    /* Finally, show what the fish has been doing. */
    if descnd {
        println!("Fish Diving");
    } else if ascend {
        println!("Fish Rising");
    } else {
        println!("Fish At Constant Depth");
    }
}

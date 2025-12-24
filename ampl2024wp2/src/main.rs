/*
 * Amplitude Hackathon Winter '24 Problem 2 - Fog Machine
 * https://dmoj.ca/problem/ampl2024wp2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the height of the smoke detector. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let detector_height = buffer.trim_end().parse::<usize>().unwrap();

    /* Calculate the heights each velocity will reach. */
    let mut v_init = 1;
    while (v_init * v_init + v_init) / 2 < detector_height {
        v_init += 1;
    }
    println!("{}", v_init - 1);
}

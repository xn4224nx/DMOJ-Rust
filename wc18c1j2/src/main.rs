/*
 * WC '18 Contest 1 J2 - Making the Cut
 * https://dmoj.ca/problem/wc18c1j2
 */

const TEAM_SIZE: usize = 5;

fn main() {
    let mut buffer = String::new();

    /* What is the applicants name. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let app_name = buffer.clone();

    /* Read the other applicants. */
    for _ in 0..TEAM_SIZE {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Is this name the same as the applicant. */
        if buffer == app_name {
            println!("Y");
            return;
        }
    }

    println!("N");
}

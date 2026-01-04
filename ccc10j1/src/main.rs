/*
 * CCC '10 J1 - What is n, Daddy?
 * https://dmoj.ca/problem/ccc10j1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the initial number. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num = buffer.trim_end().parse::<usize>().unwrap();

    println!(
        "{}",
        match num {
            1 | 9 | 10 => 1,
            2 | 3 | 7 | 8 => 2,
            4 | 5 | 6 => 3,
            _ => panic!("Number not covered!"),
        }
    );
}

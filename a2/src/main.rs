/*
 * Mirrored Pairs
 * https://dmoj.ca/problem/a2
 */

fn main() {
    let mut buffer = String::new();
    println!("Ready");

    loop {
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Check for a mirrored pair or an exit command. */
        if buffer == "bd\n" || buffer == "db\n" || buffer == "qp\n" || buffer == "pq\n" {
            println!("Mirrored pair");
        } else if buffer == "  \n" {
            break;
        } else {
            println!("Ordinary pair");
        }

        buffer.clear();
    }
}

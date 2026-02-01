/*
 * CCC '24 J1 - Conveyor Belt Sushi
 * https://dmoj.ca/problem/ccc24j1
 */

fn main() {
    let mut buffer = String::new();
    let mut plates = Vec::new();

    /* Read the number of each plate. */
    for _ in 0..3 {
        std::io::stdin().read_line(&mut buffer).unwrap();
        plates.push(buffer.trim_end().parse::<usize>().unwrap());
        buffer.clear();
    }
    println!("{}", 3 * plates[0] + 4 * plates[1] + 5 * plates[2]);
}

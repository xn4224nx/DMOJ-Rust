/*
 * An Animal Contest 1 P1 - Alpaca Shapes
 * https://dmoj.ca/problem/aac1p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the square size and circles radius. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    /* Which shape is bigger? */
    println!(
        "{}",
        if data[0] * data[0] > data[1] * data[1] * 3.14 {
            "SQUARE"
        } else {
            "CIRCLE"
        }
    );
}

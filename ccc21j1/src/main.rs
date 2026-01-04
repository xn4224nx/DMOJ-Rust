/*
 * CCC '21 J1 - Boiling Water
 * https://dmoj.ca/problem/ccc21j1
 */

const TEMP_AT_SEA_LEVEL: usize = 100;

fn main() {
    let mut buffer = String::new();

    /* What is the temperature? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let temp = buffer.trim_end().parse::<usize>().unwrap();

    /* Calculate the pressure. */
    let pressure = (5 * temp).saturating_sub(400);
    println!("{}", pressure);

    /* Is this at sea level? */
    println!(
        "{}",
        if TEMP_AT_SEA_LEVEL > temp {
            1
        } else if TEMP_AT_SEA_LEVEL < temp {
            -1
        } else {
            0
        }
    );
}

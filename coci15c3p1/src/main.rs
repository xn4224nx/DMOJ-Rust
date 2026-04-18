/*
 * COCI '15 Contest 3 #1 Pot
 * https://dmoj.ca/problem/coci15c3p1
 */

fn main() {
    let mut total = 0;
    let mut buffer = String::new();

    /* How many numbers will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let val_count = buffer.trim_end().parse::<usize>().unwrap();

    for _ in 0..val_count {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let raw = buffer.trim_end().parse::<u64>().unwrap();

        /* Extract the parts of the true number. */
        let base = raw / 10;
        let exponent = (raw % 10) as u32;

        total += base.pow(exponent);
    }

    println!("{}", total);
}

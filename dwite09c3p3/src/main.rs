/*
 * DWITE '09 R3 #3 - Binary Test Strings 2
 * https://dmoj.ca/problem/dwite09c3p3
 */

const NUM_LINES: usize = 5;

fn main() {
    let mut buffer = String::new();

    /* Read the binary filters. */
    for _ in 0..NUM_LINES {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();

        println!(
            "{}",
            (1..=u8::MAX)
                .into_iter()
                .filter(|x| !format!("{x:08b}").contains(&buffer))
                .map(|x| x.count_ones())
                .sum::<u32>()
        );
    }
}

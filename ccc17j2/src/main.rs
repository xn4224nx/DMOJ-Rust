/*
 * CCC '17 J2 - Shifty Sum
 * https://dmoj.ca/problem/ccc17j2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the original number. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num = buffer.trim().parse::<usize>().unwrap();

    /* Read the number of shifts. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n_shifts = buffer.trim().parse::<usize>().unwrap();

    /* Sum each number created by the shifts. */
    println!(
        "{}",
        (0..=n_shifts)
            .map(|x| num * 10usize.pow(x as u32))
            .sum::<usize>()
    );
}

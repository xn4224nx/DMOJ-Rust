/*
 * CCC '09 J1 - ISBN
 * https://dmoj.ca/problem/ccc09j1
 */

fn main() {
    let mut buffer = String::new();
    let mut one_three_sum = 91;

    /* Read the last three digits. */
    for rd_idx in 0..3 {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let num = buffer.trim_end().parse::<usize>().unwrap();

        one_three_sum += if rd_idx == 1 { num * 3 } else { num };
    }
    println!("The 1-3-sum is {}", one_three_sum);
}

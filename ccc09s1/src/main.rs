/*
 * CCC '09 S1 - Cool Numbers
 * https://dmoj.ca/problem/ccc09s1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the limits to search within. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Parse the limits. */
    let limits = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    /* Count the numbers that are both cube roots and square roots. */
    println!("{}", find_intresting_numbers(limits[0], limits[1]));
}

fn find_intresting_numbers(start: u32, stop: u32) -> usize {
    let mut intra_cnt = 0;

    for num in start..=stop {
        let test_num: u32 = num as u32;

        /* Check its a square number. */
        if test_num.isqrt().pow(2) != test_num {
            continue;
        }

        /* Check it's a cubed number */
        let cb_root = (test_num as f64).cbrt() as u32;

        if cb_root.pow(3) == test_num {
            intra_cnt += 1;
        }
    }
    return intra_cnt;
}

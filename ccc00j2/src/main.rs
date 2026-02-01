/*
 * CCC '00 J2 - 9966
 * https://dmoj.ca/problem/ccc00j2
 */

fn main() {
    let mut buffer = String::new();
    let rot_nums = vec![
        1, 8, 11, 69, 88, 96, 101, 111, 181, 609, 619, 689, 808, 818, 888, 906, 916, 986, 1001,
        1111, 1691, 1881, 1961, 6009, 6119, 6699, 6889, 6969, 8008, 8118, 8698, 8888, 8968, 9006,
        9116, 9696, 9886, 9966, 10001, 10101, 10801, 11011, 11111, 11811, 16091, 16191, 16891,
        18081, 18181, 18881, 19061, 19161, 19861,
    ];

    /* Read the bounds. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let lower_lim = buffer.trim_end().parse::<usize>().unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let upper_lim = buffer.trim_end().parse::<usize>().unwrap();

    /* How many rotatable numbers are there. */
    println!(
        "{}",
        rot_nums
            .iter()
            .filter(|x| **x >= lower_lim && **x <= upper_lim)
            .count()
    );
}

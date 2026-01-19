/*
 * CCC '17 J3 - Exactly Electrical
 * https://dmoj.ca/problem/ccc17j3
 */

fn main() {
    let mut buffer = String::new();

    /* Read the starting coordinate. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Read the ending coordinate. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Read the distance to verify. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Parse the data. */
    let data = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    /* Calculate the difference between the distances. */
    let tr_dist = data[4] - manhatt_dist(data[0], data[1], data[2], data[3]);

    /* Verify the distance is possible. */
    println!(
        "{}",
        if tr_dist >= 0 && tr_dist % 2 == 0 {
            "Y"
        } else {
            "N"
        }
    );
}

fn manhatt_dist(pnt0_x: i32, pnt0_y: i32, pnt1_x: i32, pnt1_y: i32) -> i32 {
    return (pnt0_x.abs_diff(pnt1_x) + pnt0_y.abs_diff(pnt1_y)) as i32;
}

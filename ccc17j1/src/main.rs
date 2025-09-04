/*
 * CCC '17 J1 - Quadrant Selection
 * https://dmoj.ca/problem/ccc17j1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the coordinates. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let x_coord = buffer.trim().parse::<i64>().unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let y_coord = buffer.trim().parse::<i64>().unwrap();

    /* Determine what quadrant the point is in. */
    let quad_num = if x_coord > 0 && y_coord > 0 {
        1
    } else if x_coord < 0 && y_coord > 0 {
        2
    } else if x_coord < 0 && y_coord < 0 {
        3
    } else if x_coord > 0 && y_coord < 0 {
        4
    } else {
        panic!("Uknown quadrant encountered!");
    };
    println!("{quad_num}");
}

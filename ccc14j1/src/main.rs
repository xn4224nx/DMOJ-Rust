/*
 * CCC '14 J1 - Triangle Times
 * https://dmoj.ca/problem/ccc14j1
 */

use std::collections::HashSet;

fn main() {
    let num_sides = 3;
    let mut buffer = String::new();
    let mut tri_perimeter = 0;
    let mut unique_sides = HashSet::with_capacity(num_sides);

    /* Read the side lengths of the triangle. */
    for _ in 0..num_sides {
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();
        let tri_side_len = buffer.parse::<usize>().unwrap();
        tri_perimeter += tri_side_len;
        unique_sides.insert(tri_side_len);
        buffer.clear()
    }
    println!("{}", triangle_type(tri_perimeter, unique_sides.len()));
}

/// Determine the type of triangle the side lengths and perimeter imply.
fn triangle_type(triangle_perimeter: usize, num_unique_sides: usize) -> String {
    if triangle_perimeter != 180 {
        return String::from("Error");
    } else if num_unique_sides == 3 {
        return String::from("Scalene");
    } else if num_unique_sides == 2 {
        return String::from("Isosceles");
    } else {
        return String::from("Equilateral");
    }
}

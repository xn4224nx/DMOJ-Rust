/*
 * IOI '94 P1 - The Triangle
 * https://dmoj.ca/problem/ioi94p1
 */

fn main() {
    let mut buffer = String::new();

    /* How many rows are in the triangle. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_rows = buffer.trim().parse::<usize>().unwrap();

    /* Allocate space for the triangle contents. */
    let mut triangle: Vec<Vec<usize>> = Vec::with_capacity(num_rows);

    /* Read the contents of the triangle. */
    for _ in 0..num_rows {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        triangle.push(
            buffer
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        );
    }

    /* Use the rows as a record of the largest path sums. Start from the
     * bottom up and pick the larger number to add to this one . */
    for row_idx in (0..num_rows - 1).rev() {
        for col_idx in 0..=row_idx {
            triangle[row_idx][col_idx] +=
                if triangle[row_idx + 1][col_idx] > triangle[row_idx + 1][col_idx + 1] {
                    triangle[row_idx + 1][col_idx]
                } else {
                    triangle[row_idx + 1][col_idx + 1]
                };
        }
    }
    println!("{}", triangle[0][0]);
}

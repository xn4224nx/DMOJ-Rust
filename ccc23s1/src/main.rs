/*
 * CCC '23 S1 - Trianglane
 * https://dmoj.ca/problem/ccc23s1
 */

fn main() {
    let mut buffer = String::new();
    let mut perim: u32 = 0;

    /* Read the number of columns. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_columns = buffer.trim().parse::<usize>().unwrap();

    /* Read the first row. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let row_0 = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    /* Read the second row. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let row_1 = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    /* Iterate left to right accross the rows summing the permimeter. */
    for col_idx in 0..num_columns {
        perim += row_0[col_idx] * 3 + row_1[col_idx] * 3;

        /* Do the traingles touch vertically. */
        if row_0[col_idx] == 1 && row_1[col_idx] == 1 && col_idx % 2 == 0 {
            perim -= 2;
        }

        /* Count instances of the triangles touching horizontally. */
        if col_idx > 0 {
            if row_0[col_idx] == 1 && row_0[col_idx - 1] == 1 {
                perim -= 2;
            }
            if row_1[col_idx] == 1 && row_1[col_idx - 1] == 1 {
                perim -= 2;
            }
        }
    }
    println!("{}", perim);
}

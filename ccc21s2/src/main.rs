/*
 * CCC '21 S2 - Modern Art
 * https://dmoj.ca/problem/ccc21s2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the dimensions of the canvas and the artist strokes. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let dims = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Keep a record of the number and position of strokes. */
    let mut row_strk = vec![0; dims[0]];
    let mut col_strk = vec![0; dims[1]];

    /* Read the strokes */
    for _ in 0..dims[2] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let rw_instrc = buffer.trim_end().split_whitespace().collect::<Vec<&str>>();

        let ele_idx = rw_instrc[1].parse::<usize>().unwrap().saturating_sub(1);

        /* Increment the record of strokes. */
        match rw_instrc[0] {
            "R" => row_strk[ele_idx] += 1,
            "C" => col_strk[ele_idx] += 1,
            _ => panic!("Unsupported command '{}'!", rw_instrc[0]),
        };
    }

    /* Count the rows and columns that are painted */
    let row_sum = row_strk.into_iter().filter(|x| x % 2 != 0).count();
    let col_sum = col_strk.into_iter().filter(|x| x % 2 != 0).count();

    /* Calculate the total painted squares. */
    println!(
        "{}",
        dims[1] * row_sum + dims[0] * col_sum - 2 * row_sum * col_sum
    );
}

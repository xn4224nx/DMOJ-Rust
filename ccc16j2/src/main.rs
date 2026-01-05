/*
 * CCC '16 J2 - Magic Squares
 * https://dmoj.ca/problem/ccc16j2
 */

const SQR_SIZE: usize = 4;

fn main() {
    let mut buffer = String::new();
    let mut sqr = Vec::new();

    /* Read the Square. */
    for _ in 0..SQR_SIZE {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        sqr.push(
            buffer
                .trim_end()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    /* Sum the first row and get the magic value. */
    let magic_val = sqr[0].iter().sum::<usize>();

    /* Ensure every other row equals this value. */
    for row_idx in 1..SQR_SIZE {
        if magic_val != sqr[row_idx].iter().sum::<usize>() {
            println!("not magic");
            return;
        }
    }

    /* Then check the columns all equal this value. */
    for col_idx in 0..SQR_SIZE {
        let col_sum = (0..SQR_SIZE).map(|x| sqr[x][col_idx]).sum::<usize>();

        if magic_val != col_sum {
            println!("not magic");
            return;
        }
    }

    /* If every row and column has the same sum it must be a magic square. */
    println!("magic");
}

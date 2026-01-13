/*
 * CCC '18 S2 - Sunflowers
 * https://dmoj.ca/problem/ccc18s2
 */

const MAX_ROTATIONS: usize = 3;

fn main() {
    let mut buffer = String::new();
    let mut sqr = Vec::new();

    /* How large is the square of sunflowers. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let sqr_size = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the rotated square of heights. */
    for _ in 0..sqr_size {
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

    /* Rotate the matrix till its is correct, then print it. */
    for _ in 0..=MAX_ROTATIONS {
        if is_matrix_correct(&sqr) {
            print_matrix(&sqr);
            break;
        }

        matrix_rot_90_clock(&mut sqr);
    }
}

fn matrix_rot_90_clock(matrix: &mut Vec<Vec<usize>>) {
    let mat_size = matrix.len();

    /* Transpose the matrix */
    for idx in 0..mat_size {
        for jdx in (idx + 1)..mat_size {
            let temp = matrix[idx][jdx];
            matrix[idx][jdx] = matrix[jdx][idx];
            matrix[jdx][idx] = temp;
        }
    }

    /* Reverse the matrix rows */
    for idx in 0..mat_size {
        for jdx in 0..(mat_size / 2) {
            let temp = matrix[idx][jdx];
            matrix[idx][jdx] = matrix[idx][mat_size - 1 - jdx];
            matrix[idx][mat_size - 1 - jdx] = temp;
        }
    }
}

/// Do the columns increase top to bottom and the rows increase left to right?
fn is_matrix_correct(matrix: &Vec<Vec<usize>>) -> bool {
    /* Check the rows. */
    for idx in 0..matrix.len() {
        for jdx in 1..matrix.len() {
            if matrix[idx][jdx] <= matrix[idx][jdx - 1] {
                return false;
            }
        }
    }

    /* Check columns. */
    for jdx in 0..matrix.len() {
        for idx in 1..matrix.len() {
            if matrix[idx][jdx] <= matrix[idx - 1][jdx] {
                return false;
            }
        }
    }
    return true;
}

fn print_matrix(matrix: &Vec<Vec<usize>>) {
    for idx in 0..matrix.len() {
        println!(
            "{}",
            matrix[idx]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

/*
 * COCI '16 Contest 7 #1 Baza
 * https://dmoj.ca/problem/coci16c7p1
 */

const CORRUPT_ENTRY: i32 = -1;

fn main() {
    let mut buffer = String::new();

    /* Read the database dimensions. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let dims = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut db_acc = Vec::new();

    /* Read the database. */
    for _ in 0..dims[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        db_acc.push(
            buffer
                .trim_end()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
    }

    /* Determine the number of Queries. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_queries = buffer.trim_end().parse::<usize>().unwrap();

    /* Read and answer the queries. */
    for _ in 0..num_queries {
        let mut matching_rows = 0;

        /* Read the query. */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let query = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        /* Determine how many rows in the db match the query. */
        'row_loop: for row in 0..dims[0] {
            for col in 0..dims[1] {
                if query[col] == CORRUPT_ENTRY {
                    continue;
                } else if query[col] != db_acc[row][col] {
                    continue 'row_loop;
                }
            }
            matching_rows += 1;
        }
        println!("{}", matching_rows);
    }
}

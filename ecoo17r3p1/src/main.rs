/*
 * ECOO '17 R3 P1 - Baker Brie
 * https://dmoj.ca/problem/ecoo17r3p1
 */

fn main() {
    let num_datasets = 10;
    let bdozen_size = 13;

    /* Read the bakery datasets. */
    for _ in 0..num_datasets {
        let mut b_dozen_cnt = 0;
        let daily_data = read_sold_goods();

        /* Sum the number of goods sold for each row. */
        for row_idx in 0..daily_data.len() {
            let row_sum = daily_data[row_idx].iter().sum::<u32>();

            /* Record the number of bakers dozens. */
            if row_sum % bdozen_size == 0 {
                b_dozen_cnt += row_sum / bdozen_size;
            }
        }

        /* Sum the number of goods sold for each column. */
        for col_idx in 0..daily_data[0].len() {
            let col_sum = (0..daily_data.len())
                .map(|row_idx| daily_data[row_idx][col_idx])
                .sum::<u32>();

            /* Record the number of bakers dozens. */
            if col_sum % bdozen_size == 0 {
                b_dozen_cnt += col_sum / bdozen_size;
            }
        }

        /* Show the total number of bakers dozens found. */
        println!("{}", b_dozen_cnt);
    }
}

/// Read from stdin a full set of bakery sales data.
fn read_sold_goods() -> Vec<Vec<u32>> {
    let mut buffer = String::new();
    let mut sold_goods: Vec<Vec<u32>> = Vec::new();

    /* Read the number of francises and the number of days. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data_dims: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    /* Read the bkery sales data line by line */
    for _ in 0..data_dims[1] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        sold_goods.push(
            buffer
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect(),
        );
    }
    return sold_goods;
}

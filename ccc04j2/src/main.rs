/*
 * CCC '04 J2 - Terms of Office
 * https://dmoj.ca/problem/ccc04j2
 */

fn main() {
    let mut buffer = String::new();
    let term_lens = vec![5, 4, 3, 2];

    /* What is the starting year. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let start_year = buffer.trim_end().parse::<usize>().unwrap();

    /* What is the end year. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let end_year = buffer.trim_end().parse::<usize>().unwrap();

    /* Print years when all the all the positions change. */
    'year_check: for year_diff in 0..=(end_year - start_year) {
        for t_len in term_lens.iter() {
            if year_diff % t_len != 0 {
                continue 'year_check;
            }
        }
        println!("All positions change in year {}", start_year + year_diff);
    }
}

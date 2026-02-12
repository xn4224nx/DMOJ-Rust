/*
 * CCC '12 S3 - Absolutely Acidic
 * https://dmoj.ca/problem/ccc12s3
 */

const MAX_INPUT_VAL: usize = 1000;

fn main() {
    let mut buffer = String::new();
    let mut val_freq = vec![0; MAX_INPUT_VAL];

    /* How many input values will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_input_val = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the stream of values */
    for _ in 0..num_input_val {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let input_val = buffer.trim_end().parse::<usize>().unwrap() - 1;
        val_freq[input_val] += 1
    }

    /* Order the numbers by frequency. */
    let mut ord_freq = (0..MAX_INPUT_VAL)
        .into_iter()
        .zip(val_freq)
        .collect::<Vec<(usize, usize)>>();
    ord_freq.sort_by_key(|&(_, x)| x);
    ord_freq.reverse();

    /* Find the two highest frequentcies. */
    let mut max_freq1 = 0;

    for idx in 1..MAX_INPUT_VAL {
        if ord_freq[idx].1 < ord_freq[0].1 {
            max_freq1 = ord_freq[idx].1;
            break;
        }
    }

    /* Find all the values with the two highest frequencies. */
    let mut max0_vals = Vec::new();
    let mut max1_vals = Vec::new();

    for idx in 0..MAX_INPUT_VAL {
        if ord_freq[idx].1 == ord_freq[0].1 {
            max0_vals.push(ord_freq[idx].0);
        } else if ord_freq[idx].1 == max_freq1 {
            max1_vals.push(ord_freq[idx].0);
        } else {
            break;
        }
    }

    /* Calculate the largest absolute difference. */
    max0_vals.sort();
    max1_vals.sort();

    let largest_abs_diff = if max0_vals.len() > 1 {
        max0_vals[0].abs_diff(max0_vals[max0_vals.len() - 1])
    } else {
        std::cmp::max(
            max0_vals[0].abs_diff(max1_vals[0]),
            max0_vals[0].abs_diff(max1_vals[max1_vals.len() - 1]),
        )
    };

    println!("{}", largest_abs_diff);
}

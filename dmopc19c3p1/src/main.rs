/*
 * DMOPC '19 Contest 3 P1 - Mode Finding
 * https://dmoj.ca/problem/dmopc19c3p1
 */

use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();

    /* How numbers in the total will be on the next line. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let total_nums = buffer.trim().parse::<usize>().unwrap();

    let mut all_nums: HashMap<i32, usize> = HashMap::with_capacity(total_nums);

    /* Read the line of numbers. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Count the appearances of each number. */
    for num in buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
    {
        all_nums.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }

    /* Find the maximum number of occurances and what that occured the most. */
    let mut modal_nums: Vec<i32> = Vec::new();
    let mut modal_ocurs: usize = 0;

    for (num, occurs) in &all_nums {
        if *occurs > modal_ocurs {
            modal_nums = vec![*num];
            modal_ocurs = *occurs;

        /* Check for draws in the mode */
        } else if *occurs == modal_ocurs {
            modal_nums.push(*num);
        }
    }

    /* Show the modal numbers */
    modal_nums.sort();
    println!(
        "{}",
        modal_nums
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

/*
 * Sorting
 * https://dmoj.ca/problem/a4b1
 */

fn main() {
    let mut buffer = String::new();

    /* How many numbers will need sorting? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.pop();
    let total_nums = buffer.parse::<usize>().unwrap();

    /* Read the unsorted numbers. */
    let mut nums: Vec<usize> = Vec::with_capacity(total_nums);
    for _ in 0..total_nums {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();
        nums.push(buffer.parse::<usize>().unwrap());
    }

    /* Sort the numbers smallest to largest. */
    nums.sort();

    /* Print the sorted array. */
    for ele in nums.into_iter() {
        println!("{}", ele);
    }
}

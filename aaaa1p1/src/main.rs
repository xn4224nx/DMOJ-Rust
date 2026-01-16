/*
 * AAAA 1 P1 - Even-Odd Sort
 * https://dmoj.ca/problem/aaaa1p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the list of numbers. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let all_nums = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!(
        "{}",
        match does_steven_win(&all_nums) {
            true => "Steven",
            false => "Todd",
        }
    )
}

fn does_steven_win(nums: &Vec<usize>) -> bool {
    /* If the list size is odd steven gets the last turn and will win. */
    if nums.len() % 2 != 0 {
        return true;
    }

    /* Count the number of odd values in the list, if its greater than half Todd wins. */
    return nums.iter().filter(|x| *x % 2 != 0).count() <= nums.len() / 2;
}

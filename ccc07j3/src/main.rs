/*
 * CCC '07 J3 - Deal or No Deal Calculator
 * https://dmoj.ca/problem/ccc07j3
 */

fn main() {
    let box_values = vec![
        100, 500, 1000, 5000, 10_000, 25_000, 50_000, 100_000, 500_000, 1_000_000,
    ];
    let all_box_total = box_values.iter().sum::<usize>();

    /* How many boxes have been opened and what values are in them. */
    let num_boxes = read_num();
    let opened_box_value = (0..num_boxes)
        .map(|_| box_values[read_num() - 1])
        .sum::<usize>();

    /* What is the offer from the banker. */
    let offer = read_num();

    /* If the offer is greater than the average of the other remaining boxes accept. */
    if offer > (all_box_total - opened_box_value) / (box_values.len() - num_boxes) {
        println!("deal");
    } else {
        println!("no deal");
    };
}

fn read_num() -> usize {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().parse::<usize>().unwrap();
}

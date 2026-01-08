/*
 * CCC '06 J2 - Roll the Dice
 * https://dmoj.ca/problem/ccc06j2
 */

const DICE_TARGET: usize = 10;

fn main() {
    let mut buffer = String::new();

    /* Read the dice sizes. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let die_0 = buffer.trim_end().parse::<usize>().unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let die_1 = buffer.trim_end().parse::<usize>().unwrap();

    /* Determine how many ways the dice can generate the target. */
    let mut way_cnt = 0;

    for roll_0 in 1..=die_0 {
        for roll_1 in 1..=die_1 {
            if roll_0 + roll_1 == DICE_TARGET {
                way_cnt += 1;
            }
        }
    }

    if way_cnt == 1 {
        println!("There is {} way to get the sum {}.", way_cnt, DICE_TARGET);
    } else {
        println!("There are {} ways to get the sum {}.", way_cnt, DICE_TARGET);
    }
}

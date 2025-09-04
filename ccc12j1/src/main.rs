/*
 * CCC '12 J1 - Speed fines are not fine!
 * https://dmoj.ca/problem/ccc12j1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the speed limit. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let speed_lim = buffer.trim_end().parse::<usize>().unwrap();
    buffer.clear();

    /* Read the car's speed. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let car_speed = buffer.trim_end().parse::<usize>().unwrap();

    /* Determine how much over the speed limit the car is going. */
    match car_speed.saturating_sub(speed_lim) {
        0 => println!("Congratulations, you are within the speed limit!"),
        1..=20 => println!("You are speeding and your fine is $100."),
        21..=30 => println!("You are speeding and your fine is $270."),
        31.. => println!("You are speeding and your fine is $500."),
    }
}
